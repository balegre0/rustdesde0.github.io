use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

async fn dato() -> u32 {
    7
}

async fn programa() {
    let n = dato().await; // `.await` dentro de una función async
    println!("{n}");
}

fn main() {
    // un ejecutor mínimo: sondea el future hasta que termina
    let mut fut = pin!(programa());
    let mut cx = Context::from_waker(Waker::noop());
    while fut.as_mut().poll(&mut cx) == Poll::Pending {}
}
