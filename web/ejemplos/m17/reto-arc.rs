use std::sync::Arc;
use std::thread;

struct Config {
    nombre: String,
    hilos: usize,
}

fn main() {
    let cfg = Arc::new(Config { nombre: String::from("servidor"), hilos: 2 });
    let mut manijas = Vec::new();
    for i in 0..cfg.hilos {
        let cfg = Arc::clone(&cfg);
        manijas.push(thread::spawn(move || format!("{} #{i}", cfg.nombre)));
    }
    for h in manijas {
        println!("{}", h.join().unwrap());
    }
}
