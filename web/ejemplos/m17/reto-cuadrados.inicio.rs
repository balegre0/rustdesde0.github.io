use std::thread;

fn main() {
    let numeros = [3, 5, 7];
    let mut manijas = Vec::new();
    for n in numeros {
        // Lanza un hilo que DEVUELVA n * n y guarda su manija en `manijas`.
        let _ = n;
        let _ = thread::current();
    }
    // Recorre `numeros` y `manijas` a la vez (zip) e imprime «3² = 9», etc.
    // Usa join().unwrap() para obtener cada resultado.
    let _ = &mut manijas as &mut Vec<thread::JoinHandle<i32>>;
}
