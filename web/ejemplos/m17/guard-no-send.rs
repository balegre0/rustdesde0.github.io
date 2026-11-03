use std::sync::Mutex;
use std::thread;

fn main() {
    let m = Mutex::new(0);
    let guard = m.lock().unwrap();
    thread::scope(|s| {
        // Intentamos llevar el guard (el candado tomado) a otro hilo.
        s.spawn(move || drop(guard));
    });
}
