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

// 1. Acepta cualquier closure async que reciba un u32 (trait AsyncFn, Rust 1.85)
async fn aplicar_dos_veces(f: impl AsyncFn(u32) -> u32, x: u32) -> u32 {
    let y = f(x).await;
    f(y).await
}

fn main() {
    let suma = 10;
    // 2. Una closure async: captura `suma` por préstamo
    let sumar = async |n: u32| n + suma;
    let r = block_on(aplicar_dos_veces(sumar, 1));
    println!("r = {r}");

    // 3. Un bloque async es un future anónimo, como una async fn sin nombre
    let bloque = async { 6 * 7 };
    println!("bloque = {}", block_on(bloque));
}
