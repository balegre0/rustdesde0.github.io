use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let total = Arc::new(Mutex::new(0));
    let mut hilos = Vec::new();
    for _ in 0..4 {
        let t = Arc::clone(&total);
        hilos.push(thread::spawn(move || {
            for _ in 0..1000 {
                *t.lock().unwrap() += 1;
            }
        }));
    }
    for h in hilos {
        h.join().unwrap();
    }
    println!("{}", *total.lock().unwrap());
}
