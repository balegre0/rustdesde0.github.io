use std::thread;

fn main() {
    let numeros: Vec<u64> = (1..=1000).collect();

    // 1. Partimos el vector en 4 trozos (préstamos de `numeros`).
    let total: u64 = thread::scope(|s| {
        let mut manijas = Vec::new();
        for trozo in numeros.chunks(250) {
            // 2. Cada hilo suma su trozo. No hace falta `move` ni `clone`.
            manijas.push(s.spawn(|| trozo.iter().sum::<u64>()));
        }
        // 3. Recogemos los resultados parciales.
        manijas.into_iter().map(|h| h.join().unwrap()).sum()
    }); // 4. Aquí TODOS los hilos han terminado, garantizado.

    println!("suma = {total}");
    println!("numeros sigue siendo mío: {} elementos", numeros.len());
}
