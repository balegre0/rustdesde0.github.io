use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let a = Arc::new(Mutex::new("cuenta A"));
    let b = Arc::new(Mutex::new("cuenta B"));

    let (a1, b1) = (Arc::clone(&a), Arc::clone(&b));
    let h1 = thread::spawn(move || {
        let _ga = a1.lock().unwrap(); // hilo 1: primero A...
        thread::sleep(Duration::from_millis(50));
        let _gb = b1.lock().unwrap(); // ...luego B
        println!("hilo 1 transfiere");
    });

    let (a2, b2) = (Arc::clone(&a), Arc::clone(&b));
    let h2 = thread::spawn(move || {
        let _gb = b2.lock().unwrap(); // hilo 2: primero B...
        thread::sleep(Duration::from_millis(50));
        let _ga = a2.lock().unwrap(); // ...luego A
        println!("hilo 2 transfiere");
    });

    h1.join().unwrap(); // nunca vuelve: cada hilo espera al otro
    h2.join().unwrap();
}
