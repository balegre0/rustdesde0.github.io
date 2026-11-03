use std::thread;

static mut CONTADOR: u32 = 0;

fn main() {
    let hilos: Vec<_> = (0..2)
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..1000 {
                    // Dos hilos escriben a la vez sin sincronizar: carrera de datos (UB).
                    unsafe { *(&raw mut CONTADOR) += 1 };
                }
            })
        })
        .collect();
    for h in hilos {
        h.join().unwrap();
    }
    println!("{}", unsafe { *(&raw const CONTADOR) });
}
