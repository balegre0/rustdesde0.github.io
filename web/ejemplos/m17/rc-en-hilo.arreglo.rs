use std::sync::Arc;
use std::thread;

fn main() {
    let r = Arc::new(5); // contador de referencias ATÓMICO
    let h = thread::spawn(move || println!("{r}"));
    h.join().unwrap();
}
