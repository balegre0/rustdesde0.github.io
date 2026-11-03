use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

fn bloquear<F: Future>(f: F) -> F::Output {
    let mut f = pin!(f);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

async fn leer_precio() -> u32 { 10 }
async fn leer_cantidad() -> u32 { 3 }

async fn total() -> u32 {
    // 1. `.await` espera a que el futuro termine y entrega su valor.
    let p = leer_precio().await;
    let c = leer_cantidad().await;
    p * c
}

fn main() {
    println!("total = {}", bloquear(total()));
}
