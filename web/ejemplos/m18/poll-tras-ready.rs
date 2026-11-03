use std::future::Future;
use std::pin::pin;
use std::task::{Context, Waker};

async fn uno() -> u8 { 1 }

fn main() {
    let mut f = pin!(uno());
    let mut cx = Context::from_waker(Waker::noop());
    println!("{:?}", f.as_mut().poll(&mut cx));
    // Sondear otra vez un future que ya devolvió Ready
    println!("{:?}", f.as_mut().poll(&mut cx));
}
