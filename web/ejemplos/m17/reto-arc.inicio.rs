use std::rc::Rc;
use std::thread;

struct Config {
    nombre: String,
    hilos: usize,
}

fn main() {
    // No compila: Rc no es Send. Cámbialo para que los hilos compartan la misma Config.
    let cfg = Rc::new(Config { nombre: String::from("servidor"), hilos: 2 });
    let mut manijas = Vec::new();
    for i in 0..cfg.hilos {
        let cfg = Rc::clone(&cfg);
        manijas.push(thread::spawn(move || format!("{} #{i}", cfg.nombre)));
    }
    for h in manijas {
        println!("{}", h.join().unwrap());
    }
}
