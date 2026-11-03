use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let productor = thread::spawn(move || {
        for n in 1..=5 {
            tx.send(n * n).unwrap();
        }
    });
    let suma: i32 = rx.iter().sum();
    productor.join().unwrap();
    println!("suma de cuadrados = {suma}");
}
