use std::cell::Cell;
use std::thread;

fn main() {
    let visitas = Cell::new(0);
    thread::scope(|s| {
        s.spawn(|| visitas.set(visitas.get() + 1));
        s.spawn(|| visitas.set(visitas.get() + 1));
    });
    println!("{}", visitas.get());
}
