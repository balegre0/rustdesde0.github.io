use std::future::Future;
use std::task::{Context, Waker};

async fn tarea() -> u8 { 1 }

fn main() {
    let mut fut = tarea();
    let mut cx = Context::from_waker(Waker::noop());
    // poll pide `self: Pin<&mut Self>`, no `&mut self`
    let _ = fut.poll(&mut cx);
}
