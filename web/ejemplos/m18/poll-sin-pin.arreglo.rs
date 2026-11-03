use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

async fn tarea() -> u8 { 1 }

fn main() {
    // pin! fija el future en la pila: ya no se puede mover
    let mut fut = pin!(tarea());
    let mut cx = Context::from_waker(Waker::noop());
    if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
        println!("listo: {v}");
    }
}
