use std::thread;

fn main() {
    let notas = vec![7, 9, 5, 10];
    let mitad = notas.len() / 2;
    let (a, b) = notas.split_at(mitad);
    // thread::scope: todos los hilos terminan antes de salir del bloque,
    // así que pueden tomar prestados `a` y `b`.
    let total = thread::scope(|s| {
        let h1 = s.spawn(|| a.iter().sum::<i32>());
        let h2 = s.spawn(|| b.iter().sum::<i32>());
        h1.join().unwrap() + h2.join().unwrap()
    });
    println!("{total}");
}
