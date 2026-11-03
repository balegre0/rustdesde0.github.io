use std::thread;

fn main() {
    let mut hilos = Vec::new();
    for i in 1..=3 {
        // move: el hilo se lleva su propia copia de `i`
        hilos.push(thread::spawn(move || i * i));
    }
    // join espera a que termine y devuelve el resultado
    let cuadrados: Vec<i32> = hilos.into_iter().map(|h| h.join().unwrap()).collect();
    println!("{cuadrados:?}");

    let h = thread::spawn(|| panic!("algo salió mal"));
    println!("¿el hilo entró en pánico? {}", h.join().is_err());
}
