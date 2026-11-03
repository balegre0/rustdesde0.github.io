use std::sync::atomic::{AtomicU32, Ordering};

// Incrementa `c` solo si no ha llegado al tope. Devuelve si lo consiguió.
// Pista: compare_exchange en bucle, o try_update.
fn incrementar_hasta(c: &AtomicU32, tope: u32) -> bool {
    let _ = (c, tope, Ordering::Relaxed);
    false
}

fn main() {
    let entradas = AtomicU32::new(0);
    for _ in 0..5 {
        print!("{} ", incrementar_hasta(&entradas, 3));
    }
    println!();
    println!("entradas vendidas: {}", entradas.load(Ordering::Relaxed));
}
