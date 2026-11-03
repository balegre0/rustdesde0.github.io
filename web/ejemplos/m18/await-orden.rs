use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

async fn paso(nombre: &str) -> usize {
    println!("  empieza {nombre}");
    nombre.len()
}

fn main() {
    block_on(async {
        // 1. Crear dos futures no ejecuta nada
        let a = paso("uno");
        let b = paso("tres");
        println!("futures creados");
        // 2. .await los ejecuta, en el orden en que esperas
        let nb = b.await;
        let na = a.await;
        println!("total = {}", na + nb);
    });
}
