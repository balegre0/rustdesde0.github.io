use std::thread;

fn main() {
    let datos = vec![1, 2, 3];
    let h = thread::spawn(move || {
        let suma: i32 = datos.iter().sum();
        suma
    });
    println!("datos = {datos:?}"); // `datos` ya se movió al hilo
    println!("suma = {}", h.join().unwrap());
}
