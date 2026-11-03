use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    let compartido = Arc::new(vec![1, 2, 3]); // Arc<Vec<i32>> es Send + Sync
    let copia = Arc::clone(&compartido);
    let h = thread::spawn(move || copia.len());
    println!("{}", h.join().unwrap());
    let local = Rc::new(5); // Rc no es Send: no puede cruzar a otro hilo
    println!("{local}");
}
