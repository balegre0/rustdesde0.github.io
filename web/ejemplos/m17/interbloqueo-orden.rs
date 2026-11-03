use std::sync::{Arc, Mutex};
use std::thread;

fn transferir(origen: &Mutex<i32>, destino: &Mutex<i32>, primero_origen: bool, cantidad: i32) {
    // Regla: TODOS los hilos toman los candados en el mismo orden global.
    let (mut g1, mut g2);
    if primero_origen {
        g1 = origen.lock().unwrap();
        g2 = destino.lock().unwrap();
    } else {
        g2 = destino.lock().unwrap();
        g1 = origen.lock().unwrap();
    }
    *g1 -= cantidad;
    *g2 += cantidad;
}

fn main() {
    let a = Arc::new(Mutex::new(100));
    let b = Arc::new(Mutex::new(100));
    let mut hs = Vec::new();
    for i in 0..10 {
        let (a, b) = (Arc::clone(&a), Arc::clone(&b));
        hs.push(thread::spawn(move || {
            // A siempre va antes que B, sea cual sea la dirección.
            if i % 2 == 0 {
                transferir(&a, &b, true, 5);
            } else {
                transferir(&b, &a, false, 3);
            }
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    println!("A = {}, B = {}", a.lock().unwrap(), b.lock().unwrap());
}
