use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

type Tarea = Pin<Box<dyn Future<Output = ()>>>;

// 1. El waker de cada tarea sabe su número y lo mete en la cola de listas
struct Despertador {
    id: usize,
    listas: Arc<Mutex<VecDeque<usize>>>,
}
impl Wake for Despertador {
    fn wake(self: Arc<Self>) {
        self.listas.lock().unwrap().push_back(self.id);
    }
}

// Cede el turno una vez
struct Ceder(bool);
impl Future for Ceder {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 { return Poll::Ready(()); }
        self.0 = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

async fn trabajador(nombre: &'static str, pasos: u32) {
    for i in 1..=pasos {
        println!("{nombre}: paso {i}");
        Ceder(false).await; // deja que otras tareas avancen
    }
    println!("{nombre}: fin");
}

fn main() {
    // 2. Las tareas, y la cola con las que están listas para avanzar
    let mut tareas: Vec<Option<Tarea>> = vec![
        Some(Box::pin(trabajador("ana", 2))),
        Some(Box::pin(trabajador("leo", 3))),
    ];
    let listas = Arc::new(Mutex::new(VecDeque::from([0, 1])));

    // 3. Bucle del ejecutor: saca una tarea lista, la sondea, repite
    loop {
        let siguiente = listas.lock().unwrap().pop_front();
        let Some(id) = siguiente else { break };
        let waker = Waker::from(Arc::new(Despertador { id, listas: listas.clone() }));
        let mut cx = Context::from_waker(&waker);
        if let Some(t) = tareas[id].as_mut() {
            if t.as_mut().poll(&mut cx).is_ready() {
                tareas[id] = None; // terminada: se suelta
            }
        }
    }
    println!("cola vacía: todas las tareas terminaron");
}
