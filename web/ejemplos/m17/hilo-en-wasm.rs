use std::thread;

fn main() {
    println!("Antes de crear el hilo");
    // En el navegador (wasm32-wasip1) no hay hilos del sistema operativo.
    let h = thread::spawn(|| println!("Hola desde otro hilo"));
    h.join().unwrap();
    println!("Después de join");
}
