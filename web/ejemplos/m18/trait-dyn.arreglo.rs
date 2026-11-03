use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// Versión compatible con dyn: el método devuelve un future en una caja
trait Almacen {
    fn leer(&self) -> Pin<Box<dyn Future<Output = u32> + '_>>;
}

struct Memoria;
impl Almacen for Memoria {
    fn leer(&self) -> Pin<Box<dyn Future<Output = u32> + '_>> {
        Box::pin(async { 1 })
    }
}

fn main() {
    let a: Box<dyn Almacen> = Box::new(Memoria);
    let mut fut = a.leer();
    let mut cx = Context::from_waker(Waker::noop());
    if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
        println!("leído: {v}");
    }
}
