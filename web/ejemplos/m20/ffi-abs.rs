// Llamar a funciones de la biblioteca C del sistema a través de la ABI de C.
use std::ffi::{c_char, c_int, CStr};

unsafe extern "C" {
    // 1. `safe fn`: prometemos que llamarla nunca causa UB con ningún argumento.
    safe fn abs(x: c_int) -> c_int;
    // 2. `fn` (unsafe por defecto): exige un puntero válido a texto terminado en NUL.
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    println!("abs(-7) = {}", abs(-7)); // sin unsafe gracias a `safe fn`

    let saludo: &CStr = c"hola, ñandú"; // literal C: termina en un byte NUL
    // SAFETY: saludo es un CStr válido y vive durante toda la llamada.
    let n = unsafe { strlen(saludo.as_ptr()) };
    println!("strlen = {n} bytes; en Rust: {} bytes", saludo.to_bytes().len());
}
