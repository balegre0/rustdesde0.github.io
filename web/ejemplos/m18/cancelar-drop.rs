use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// Un future que nunca termina por sí solo
struct Eterno;
impl Future for Eterno {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        Poll::Pending
    }
}

// Un guardián: su Drop nos dice cuándo se suelta
struct Guardia(&'static str);
impl Drop for Guardia {
    fn drop(&mut self) {
        println!("  drop de {}", self.0);
    }
}

async fn tarea() {
    let _g = Guardia("guardia de tarea");
    println!("  tarea: antes del .await");
    Eterno.await;
    println!("  tarea: después del .await (nunca se imprime)");
}

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = Box::pin(tarea());
    // 1. Dos sondeos: la tarea se queda parada en el .await
    for i in 1..=2 {
        println!("sondeo {i}: {:?}", fut.as_mut().poll(&mut cx).is_pending());
    }
    // 2. Cancelar = soltar el future. Se ejecutan los Drop de lo que guardaba
    println!("suelto el future");
    drop(fut);
    println!("fin de main");
}
