use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let cuenta = Arc::new(Mutex::new(0i64));
    let mut manijas: Vec<thread::JoinHandle<()>> = Vec::new();
    for i in 1..=4 {
        // Lanza un hilo que ingrese i * 10 en la cuenta.
        let _ = i;
    }
    for h in manijas.drain(..) {
        h.join().unwrap();
    }
    println!("saldo = {}", cuenta.lock().unwrap());
}
