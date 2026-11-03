// thread::scope: el diseño a prueba de fugas que sustituyó a thread::scoped.
use std::thread;

fn main() {
    let mut datos = vec![1, 2, 3, 4];
    let (izq, der) = datos.split_at_mut(2);
    thread::scope(|s| {
        // Los hilos toman prestados datos locales: el scope ESPERA a ambos
        // antes de volver, sin depender de ningún destructor.
        s.spawn(|| izq.iter_mut().for_each(|x| *x *= 10));
        s.spawn(|| der.iter_mut().for_each(|x| *x += 1));
    });
    println!("{datos:?}");
}
