use std::rc::Rc;
use std::thread;

fn main() {
    let dato = Rc::new(5);
    let copia = Rc::clone(&dato);
    // Intentamos llevar el Rc a otro hilo.
    let h = thread::spawn(move || {
        println!("{copia}");
    });
    h.join().unwrap();
}
