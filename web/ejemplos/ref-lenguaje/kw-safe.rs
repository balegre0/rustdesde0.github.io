// `safe` solo es palabra clave dentro de un bloque `unsafe extern`.
// Declaramos funciones de la ABI "C" implementadas en este mismo programa
// con `#[unsafe(no_mangle)]`, para poder llamarlas desde el bloque.

#[unsafe(no_mangle)]
pub extern "C" fn ref_doble(x: i32) -> i32 {
    x * 2
}

#[unsafe(no_mangle)]
pub extern "C" fn ref_primero(p: *const i32) -> i32 {
    // SAFETY: quien llama garantiza que `p` apunta a un i32 válido.
    unsafe { *p }
}

mod ffi {
    unsafe extern "C" {
        // `safe`: prometemos que cualquier llamada con un i32 es segura.
        pub safe fn ref_doble(x: i32) -> i32;
        // `unsafe`: quien llame debe garantizar que el puntero es válido.
        pub unsafe fn ref_primero(p: *const i32) -> i32;
    }
}

fn main() {
    println!("{}", ffi::ref_doble(21)); // sin bloque unsafe
    let v = [7, 8];
    // SAFETY: `v.as_ptr()` apunta al primer elemento de un array vivo.
    println!("{}", unsafe { ffi::ref_primero(v.as_ptr()) });
    // Fuera de un bloque extern, `safe` es un nombre normal.
    let safe = true;
    println!("{safe}");
}
