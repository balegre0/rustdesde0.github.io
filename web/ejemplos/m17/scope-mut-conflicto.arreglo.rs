use std::thread;

fn main() {
    let mut puntos = vec![1, 2, 3];
    let (izq, der) = puntos.split_at_mut(1);
    thread::scope(|s| {
        s.spawn(|| izq[0] *= 10); // cada hilo toca una parte DISTINTA
        s.spawn(|| der[1] *= 10);
    });
    println!("{puntos:?}");
}
