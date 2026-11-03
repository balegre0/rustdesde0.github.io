use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let inicio = Instant::now();
    thread::sleep(Duration::from_millis(50));
    let pasado = inicio.elapsed();
    // dormir garantiza «al menos» ese tiempo
    println!("{}", pasado >= Duration::from_millis(50));
    println!("{:?}", thread::current().name());
}
