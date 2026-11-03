use std::thread;

fn main() {
    let datos = vec![1, 2, 3];
    let copia = datos.clone(); // el hilo se lleva una copia; main conserva el original
    let h = thread::spawn(move || {
        let suma: i32 = copia.iter().sum();
        suma
    });
    println!("datos = {datos:?}");
    println!("suma = {}", h.join().unwrap());
}
