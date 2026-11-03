use std::thread;

fn main() {
    println!("antes de crear el hilo");
    let h = thread::spawn(|| 1 + 1);
    println!("{}", h.join().unwrap());
}
