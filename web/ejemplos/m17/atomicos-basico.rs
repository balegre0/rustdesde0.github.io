use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

fn main() {
    let visitas = AtomicU64::new(0);

    // 1. fetch_add: suma y devuelve el valor ANTERIOR, en un solo paso indivisible.
    let antes = visitas.fetch_add(5, Ordering::Relaxed);
    println!("antes = {antes}, ahora = {}", visitas.load(Ordering::Relaxed));

    // 2. compare_exchange: «si vale 5, ponlo a 10». Si no, dime cuánto vale.
    println!("{:?}", visitas.compare_exchange(5, 10, Ordering::SeqCst, Ordering::SeqCst));
    println!("{:?}", visitas.compare_exchange(5, 99, Ordering::SeqCst, Ordering::SeqCst));

    // 3. swap: pone un valor nuevo y devuelve el viejo.
    let parar = AtomicBool::new(false);
    println!("parar era {}", parar.swap(true, Ordering::Relaxed));

    // 4. try_update: un bucle de compare_exchange ya escrito por ti (antes se llamaba fetch_update).
    let r = visitas.try_update(Ordering::SeqCst, Ordering::SeqCst, |v| v.checked_mul(3));
    println!("{r:?} -> {}", visitas.load(Ordering::SeqCst));
}
