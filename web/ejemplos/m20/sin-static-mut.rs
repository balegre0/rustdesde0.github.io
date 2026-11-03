// Este contador global usa `static mut` y unsafe. Cámbialo por un AtomicU32
// (std::sync::atomic) para que NO quede ningún unsafe en el programa.
use std::sync::atomic::{AtomicU32, Ordering};

static SIGUIENTE_ID: AtomicU32 = AtomicU32::new(100);

fn nuevo_id() -> u32 {
    // fetch_add devuelve el valor ANTERIOR y suma de forma atómica.
    SIGUIENTE_ID.fetch_add(1, Ordering::Relaxed)
}

fn main() {
    let a = nuevo_id();
    let b = nuevo_id();
    let c = nuevo_id();
    println!("{a} {b} {c}");
}
