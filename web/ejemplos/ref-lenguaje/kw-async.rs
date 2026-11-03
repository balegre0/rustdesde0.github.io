use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

// Un ejecutor mínimo: sondea el futuro hasta que termina.
// (std no trae ejecutor; en programas reales se usa una biblioteca).
fn bloquear<F: Future>(f: F) -> F::Output {
    let mut f = pin!(f);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

// 1. Una función async devuelve un futuro: llamarla no ejecuta el cuerpo.
async fn doble(x: u32) -> u32 {
    println!("calculando el doble de {x}");
    x * 2
}

fn main() {
    let futuro = doble(21);
    println!("futuro creado");
    // 2. Al sondearlo se ejecuta el cuerpo.
    println!("{}", bloquear(futuro));
    // 3. Un bloque async es un futuro anónimo.
    let saludo = async { "hola desde un bloque async" };
    println!("{}", bloquear(saludo));
    // 4. Closure async (desde Rust 1.85).
    let suma = async |a: u32, b: u32| a + b;
    println!("{}", bloquear(suma(2, 3)));
}
