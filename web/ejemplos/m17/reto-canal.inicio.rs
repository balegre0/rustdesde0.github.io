use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let productor = thread::spawn(move || {
        // Envía los cuadrados de 1 a 5 por el canal.
        let _ = &tx;
    });
    // Suma todo lo que llegue por `rx` (el bucle acaba cuando se cierra el canal).
    let suma: i32 = 0;
    let _ = &rx;
    productor.join().unwrap();
    println!("suma de cuadrados = {suma}");
}
