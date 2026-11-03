use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

static CONTADOR: AtomicU32 = AtomicU32::new(0);

fn main() {
    let hilos: Vec<_> = (0..2)
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..1000 {
                    // Un atómico: cada suma es indivisible y no hay carrera de datos.
                    CONTADOR.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for h in hilos {
        h.join().unwrap();
    }
    println!("{}", CONTADOR.load(Ordering::Relaxed));
}
