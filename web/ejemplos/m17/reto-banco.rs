use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let cuenta = Arc::new(Mutex::new(0i64));
    let mut manijas = Vec::new();
    for i in 1..=4 {
        let c = Arc::clone(&cuenta);
        manijas.push(thread::spawn(move || {
            *c.lock().unwrap() += i * 10;
        }));
    }
    for h in manijas {
        h.join().unwrap();
    }
    println!("saldo = {}", cuenta.lock().unwrap());
}
