use std::thread;

fn main() {
    let mut contador = 0;
    thread::scope(|s| {
        s.spawn(|| contador += 1);
        s.spawn(|| contador += 1);
    });
    println!("{contador}");
}
