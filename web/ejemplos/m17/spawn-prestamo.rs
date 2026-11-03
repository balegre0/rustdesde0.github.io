use std::thread;

fn main() {
    let notas = vec![7, 9, 5, 10];
    let mitad = notas.len() / 2;
    let (a, b) = notas.split_at(mitad);
    // `a` y `b` son préstamos de `notas`, y thread::spawn exige 'static.
    let h1 = thread::spawn(move || a.iter().sum::<i32>());
    let h2 = thread::spawn(move || b.iter().sum::<i32>());
    println!("{}", h1.join().unwrap() + h2.join().unwrap());
}
