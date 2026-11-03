use std::thread;

fn main() {
    let h = thread::spawn(|| println!("hola desde otro hilo"));
    h.join().unwrap();
    println!("fin");
}
