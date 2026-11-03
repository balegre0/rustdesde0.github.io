use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(id * 10).unwrap();
        });
    }
    drop(tx); // si no, el for de abajo esperaría para siempre
    let mut recibidos: Vec<i32> = rx.iter().collect();
    recibidos.sort(); // el orden de llegada depende de los hilos
    println!("{recibidos:?}");
}
