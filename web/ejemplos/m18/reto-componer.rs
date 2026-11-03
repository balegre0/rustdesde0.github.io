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

// Aplica primero `f` y después `g`, esperando cada resultado
async fn componer(f: impl AsyncFn(u32) -> u32, g: impl AsyncFn(u32) -> u32, x: u32) -> u32 {
    let y = f(x).await;
    g(y).await
}

fn main() {
    let doble = async |n: u32| n * 2;
    let mas_uno = async |n: u32| n + 1;
    println!("{}", block_on(componer(doble, mas_uno, 20)));
}
