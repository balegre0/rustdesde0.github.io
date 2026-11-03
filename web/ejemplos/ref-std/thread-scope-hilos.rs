use std::thread;

fn main() {
    let datos = vec![1, 2, 3, 4, 5, 6];
    let (izq, der) = datos.split_at(3);
    // los hilos de un scope pueden tomar prestado: terminan antes de salir
    let (a, b) = thread::scope(|s| {
        let h1 = s.spawn(|| izq.iter().sum::<i32>());
        let h2 = s.spawn(|| der.iter().sum::<i32>());
        (h1.join().unwrap(), h2.join().unwrap())
    });
    println!("{a} + {b} = {}", a + b);
    println!("{datos:?}"); // `datos` sigue siendo nuestro
}
