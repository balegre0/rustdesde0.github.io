use std::thread;

fn main() {
    let nombre = String::from("Ana");
    // El closure toma `nombre` PRESTADO... pero el hilo podría vivir más que main.
    let h = thread::spawn(|| {
        println!("hola, {nombre}");
    });
    h.join().unwrap();
}
