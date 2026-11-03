use std::sync::atomic::{AtomicU32, Ordering};

// 1. `static`: una única posición de memoria que vive todo el programa.
static SALUDO: &str = "hola";

// 2. Mutable de forma segura: tipos con mutabilidad interior (aquí un atómico).
static VISITAS: AtomicU32 = AtomicU32::new(0);

fn visitar() -> u32 {
    VISITAS.fetch_add(1, Ordering::Relaxed) + 1
}

fn main() {
    println!("{SALUDO}");
    visitar();
    visitar();
    println!("visitas: {}", visitar());
    // 3. Siempre la misma dirección (una `const` no la tiene).
    let a: *const &str = &SALUDO;
    let b: *const &str = &SALUDO;
    println!("{}", a == b);
    // 4. `'static` en un tipo: «puede vivir todo el programa».
    let s: &'static str = "literal guardado en el binario";
    println!("{}", s.len());
}
