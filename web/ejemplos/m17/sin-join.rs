use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..=5 {
            println!("hilo: paso {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });
    println!("main: termino");
    // main termina aquí y el proceso entero acaba,
    // aunque el otro hilo no haya llegado al paso 5.
}
