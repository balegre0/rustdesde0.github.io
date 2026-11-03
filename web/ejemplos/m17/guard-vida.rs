use std::sync::Mutex;

fn main() {
    let m = Mutex::new(0);
    // Queremos «tener el candado» durante el resto de main...
    let _ = m.lock().unwrap();
    println!("¿libre? {}", m.try_lock().is_ok());
}
