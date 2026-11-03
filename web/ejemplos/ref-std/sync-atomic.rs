use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static VISITAS: AtomicUsize = AtomicUsize::new(0);

fn main() {
    for _ in 0..3 {
        VISITAS.fetch_add(1, Ordering::Relaxed);
    }
    println!("{}", VISITAS.load(Ordering::Relaxed));
    let anterior = VISITAS.swap(100, Ordering::Relaxed);
    println!("{anterior} -> {}", VISITAS.load(Ordering::Relaxed));
    // compare_exchange: cambia solo si el valor es el esperado
    println!("{:?}", VISITAS.compare_exchange(100, 7, Ordering::SeqCst, Ordering::SeqCst));
    println!("{:?}", VISITAS.compare_exchange(100, 8, Ordering::SeqCst, Ordering::SeqCst));
    let listo = AtomicBool::new(false);
    listo.store(true, Ordering::Release);
    println!("{}", listo.load(Ordering::Acquire));
}
