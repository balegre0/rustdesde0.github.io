// Declara `labs` (valor absoluto de un c_long) de la biblioteca C como `safe fn`
// dentro de un bloque `unsafe extern "C"` y úsala sin bloque unsafe.
use std::ffi::c_long;

// 1. escribe aquí el bloque unsafe extern "C" { ... }

fn absoluto(x: c_long) -> c_long {
    todo!("llama a labs")
}

fn main() {
    for x in [-5, 0, 12] {
        println!("|{x}| = {}", absoluto(x));
    }
}
