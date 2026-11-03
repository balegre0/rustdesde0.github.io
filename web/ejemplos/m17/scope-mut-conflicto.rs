use std::thread;

fn main() {
    let mut puntos = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| puntos.push(4));         // préstamo exclusivo en un hilo...
        s.spawn(|| println!("{puntos:?}")); // ...y compartido en otro, a la vez
    });
}
