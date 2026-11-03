use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

async fn saludar() -> u32 {
    println!("  (dentro de saludar)");
    7
}

fn main() {
    // 1. Llamar a una async fn NO ejecuta su cuerpo: devuelve un future
    println!("1. llamo a saludar()");
    let fut = saludar();
    println!("2. ya tengo el future, pero no ha pasado nada");

    // 2. Sondearlo (poll) es lo que ejecuta el cuerpo
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    println!("3. lo sondeo");
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => println!("4. listo: {v}"),
        Poll::Pending => println!("4. todavía no"),
    }
}
