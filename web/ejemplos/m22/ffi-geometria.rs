// Llama a tres funciones de una biblioteca escrita en C (geometria.c).
use std::ffi::c_double;

// 1. El mismo layout que el struct de la biblioteca: campos en orden, sin reordenar
#[repr(C)]
struct Punto {
    x: c_double,
    y: c_double,
}

// 2. Las firmas de las funciones de fuera. El compilador NO puede comprobarlas:
//    por eso el bloque es `unsafe extern`.
unsafe extern "C" {
    fn distancia2(a: *const Punto, b: *const Punto) -> c_double;
    fn suma_array(v: *const i32, n: usize) -> i32;
    fn aplica_dos_veces(f: extern "C" fn(i32) -> i32, x: i32) -> i32;
}

// 3. Envoltorios seguros: el `unsafe` queda encerrado aquí, con su justificación
fn distancia_al_cuadrado(a: &Punto, b: &Punto) -> f64 {
    // SAFETY: a y b son referencias válidas a Punto durante toda la llamada.
    unsafe { distancia2(a, b) }
}

fn suma(v: &[i32]) -> i32 {
    // SAFETY: el slice garantiza v.len() enteros válidos a partir de v.as_ptr().
    unsafe { suma_array(v.as_ptr(), v.len()) }
}

// 4. Una función de Rust con la ABI de C, para pasarla como callback
extern "C" fn triple(x: i32) -> i32 {
    x * 3
}

fn main() {
    let a = Punto { x: 0.0, y: 0.0 };
    let b = Punto { x: 3.0, y: 4.0 };
    println!("distancia² = {}", distancia_al_cuadrado(&a, &b));
    println!("suma = {}", suma(&[1, 2, 3, 4]));
    // SAFETY: triple tiene exactamente la firma que espera la biblioteca.
    println!("triple(triple(5)) = {}", unsafe { aplica_dos_veces(triple, 5) });
}
