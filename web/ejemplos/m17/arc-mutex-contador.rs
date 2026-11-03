use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let mut manijas = Vec::new();

    for _ in 0..8 {
        // 1. Cada hilo recibe su propio Arc (mismo Mutex, contador +1).
        let c = Arc::clone(&contador);
        manijas.push(thread::spawn(move || {
            for _ in 0..1000 {
                // 2. Leer, sumar y escribir ocurre con el candado tomado.
                *c.lock().unwrap() += 1;
            }
        }));
    }
    for h in manijas {
        h.join().unwrap();
    }
    // 3. Ninguna suma se pierde: 8 hilos × 1000 = 8000, siempre.
    println!("contador = {}", *contador.lock().unwrap());
}
