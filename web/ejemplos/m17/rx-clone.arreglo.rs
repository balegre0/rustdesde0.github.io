use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    // Si de verdad hacen falta varios consumidores: compartir el Receiver con candado.
    let rx = Arc::new(Mutex::new(rx));
    let rx2 = Arc::clone(&rx);
    tx.send(7).unwrap();
    println!("{}", rx2.lock().unwrap().recv().unwrap());
    let _ = rx;
}
