use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

async fn tarea() -> u8 { 1 }

fn main() {
    // Box::pin: el future vive en el heap y su dirección ya no cambia
    let mut fut: Pin<Box<dyn Future<Output = u8>>> = Box::pin(tarea());
    let mut cx = Context::from_waker(Waker::noop());
    if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
        println!("listo: {v}");
    }
}
