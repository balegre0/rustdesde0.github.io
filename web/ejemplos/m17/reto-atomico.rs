use std::sync::atomic::{AtomicU32, Ordering};

// Incrementa `c` solo si no ha llegado al tope. Devuelve si lo consiguió.
fn incrementar_hasta(c: &AtomicU32, tope: u32) -> bool {
    c.try_update(Ordering::Relaxed, Ordering::Relaxed, |v| if v < tope { Some(v + 1) } else { None })
        .is_ok()
}

fn main() {
    let entradas = AtomicU32::new(0);
    for _ in 0..5 {
        print!("{} ", incrementar_hasta(&entradas, 3));
    }
    println!();
    println!("entradas vendidas: {}", entradas.load(Ordering::Relaxed));
}
