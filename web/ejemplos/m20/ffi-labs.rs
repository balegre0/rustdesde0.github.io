// Declara `labs` (valor absoluto de un c_long) de la biblioteca C como `safe fn`
// dentro de un bloque `unsafe extern "C"` y úsala sin bloque unsafe.
use std::ffi::c_long;

unsafe extern "C" {
    safe fn labs(x: c_long) -> c_long;
}

fn absoluto(x: c_long) -> c_long {
    labs(x)
}

fn main() {
    for x in [-5, 0, 12] {
        println!("|{x}| = {}", absoluto(x));
    }
}
