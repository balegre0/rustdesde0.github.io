use std::thread;

fn main() {
    let numeros = [3, 5, 7];
    let mut manijas = Vec::new();
    for n in numeros {
        manijas.push(thread::spawn(move || n * n));
    }
    for (n, h) in numeros.iter().zip(manijas) {
        println!("{n}² = {}", h.join().unwrap());
    }
}
