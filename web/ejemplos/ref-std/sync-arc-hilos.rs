use std::sync::Arc;
use std::thread;

fn main() {
    let datos = Arc::new(vec![1, 2, 3]);
    let mut hilos = Vec::new();
    for i in 0..3 {
        let mio = Arc::clone(&datos);
        hilos.push(thread::spawn(move || mio[i] * 10));
    }
    let resultados: Vec<i32> = hilos.into_iter().map(|h| h.join().unwrap()).collect();
    println!("{resultados:?}");
}
