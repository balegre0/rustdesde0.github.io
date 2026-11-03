use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

// ---------- El reloj virtual y sus alarmas ----------
thread_local! {
    static AHORA: Cell<u64> = Cell::new(0);
    static ALARMAS: RefCell<Vec<(u64, Waker)>> = RefCell::new(Vec::new());
}
fn ahora() -> u64 { AHORA.with(|a| a.get()) }

// 1. dormir(t): Pending hasta que el reloj llegue a `hasta`
struct Dormir { hasta: u64 }
impl Future for Dormir {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // Si ya es la hora (ahora() >= self.hasta): Ready.
        // Si no: guarda (self.hasta, cx.waker().clone()) en ALARMAS y devuelve Pending.
        todo!()
    }
}
fn dormir(tics: u64) -> Dormir { Dormir { hasta: ahora() + tics } }

// ---------- El ejecutor ----------
type Tarea = Pin<Box<dyn Future<Output = ()>>>;
struct Despertador { id: usize, listas: Arc<Mutex<VecDeque<usize>>> }
impl Wake for Despertador {
    fn wake(self: Arc<Self>) { self.listas.lock().unwrap().push_back(self.id); }
}

fn ejecutar(tareas: Vec<Tarea>) {
    let mut tareas: Vec<Option<Tarea>> = tareas.into_iter().map(Some).collect();
    let listas = Arc::new(Mutex::new((0..tareas.len()).collect::<VecDeque<_>>()));
    loop {
        // 2. Sondea todo lo que esté listo
        loop {
            let siguiente = listas.lock().unwrap().pop_front();
            let Some(id) = siguiente else { break };
            let waker = Waker::from(Arc::new(Despertador { id, listas: listas.clone() }));
            let mut cx = Context::from_waker(&waker);
            if let Some(t) = tareas[id].as_mut() {
                if t.as_mut().poll(&mut cx).is_ready() { tareas[id] = None; }
            }
        }
        // 3. Nadie listo: salta el reloj a la próxima alarma y despierta a quien toque
        let proxima = ALARMAS.with(|al| al.borrow().iter().map(|(t, _)| *t).min());
        let Some(t) = proxima else { break };
        AHORA.with(|a| a.set(t));
        let vencidas: Vec<Waker> = ALARMAS.with(|al| {
            let mut al = al.borrow_mut();
            let (si, no): (Vec<_>, Vec<_>) = al.drain(..).partition(|(h, _)| *h <= t);
            *al = no;
            si.into_iter().map(|(_, w)| w).collect()
        });
        for w in vencidas { w.wake(); }
    }
}

// ---------- El desayuno ----------
async fn paso(que: &str, tics: u64) {
    println!("[t={}] empiezo {que}", ahora());
    dormir(tics).await;
    println!("[t={}] {que} listo", ahora());
}

fn main() {
    ejecutar(vec![
        Box::pin(paso("tostadas", 3)),
        Box::pin(async { paso("café", 1).await; paso("leche", 1).await; }),
        Box::pin(paso("huevos", 2)),
    ]);
    println!("desayuno servido en t={}", ahora());
}
