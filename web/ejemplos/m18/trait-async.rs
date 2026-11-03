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

// 1. async fn dentro de un trait (estable desde Rust 1.75)
trait Almacen {
    async fn leer(&self, clave: &str) -> Option<u32>;
}

struct Memoria;
impl Almacen for Memoria {
    async fn leer(&self, clave: &str) -> Option<u32> {
        if clave == "edad" { Some(30) } else { None }
    }
}

struct Fijo(u32);
impl Almacen for Fijo {
    async fn leer(&self, _clave: &str) -> Option<u32> {
        Some(self.0)
    }
}

// 2. Uso genérico: despacho estático, sin Box y sin coste extra
async fn edad_o_cero<A: Almacen>(a: &A) -> u32 {
    a.leer("edad").await.unwrap_or(0)
}

fn main() {
    println!("Memoria: {}", block_on(edad_o_cero(&Memoria)));
    println!("Fijo:    {}", block_on(edad_o_cero(&Fijo(7))));
}
