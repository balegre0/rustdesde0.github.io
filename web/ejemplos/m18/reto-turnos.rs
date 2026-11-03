use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

type Tarea = Pin<Box<dyn Future<Output = ()>>>;

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

async fn contar(nombre: &'static str, hasta: u32) {
    for i in 1..=hasta {
        println!("{nombre}{i}");
        Ceder(false).await;
    }
}

// Ejecutor por turnos: saca una tarea, la sondea; si sigue Pending, vuelve al final
fn por_turnos(tareas: Vec<Tarea>) {
    let mut cola: VecDeque<Tarea> = tareas.into();
    let mut cx = Context::from_waker(Waker::noop());
    while let Some(mut t) = cola.pop_front() {
        if t.as_mut().poll(&mut cx).is_pending() {
            cola.push_back(t);
        }
    }
}

fn main() {
    por_turnos(vec![
        Box::pin(contar("a", 1)),
        Box::pin(contar("b", 3)),
        Box::pin(contar("c", 2)),
    ]);
}
