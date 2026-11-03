use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

static DATO: AtomicU64 = AtomicU64::new(0);
static LISTO: AtomicBool = AtomicBool::new(false);

fn main() {
    let productor = thread::spawn(|| {
        DATO.store(42, Ordering::Relaxed); // 1. preparo el dato
        LISTO.store(true, Ordering::Release); // 2. «publico»: todo lo anterior queda visible
    });

    // 3. Espero la bandera. Acquire empareja con el Release de arriba.
    while !LISTO.load(Ordering::Acquire) {
        std::hint::spin_loop();
    }
    // 4. Si vi LISTO == true, tengo garantizado ver DATO == 42.
    println!("dato = {}", DATO.load(Ordering::Relaxed));
    productor.join().unwrap();
}
