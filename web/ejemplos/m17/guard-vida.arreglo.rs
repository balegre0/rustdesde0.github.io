use std::sync::Mutex;

fn main() {
    let m = Mutex::new(0);
    // `_g` SÍ enlaza: el guard vive hasta el final del bloque.
    let _g = m.lock().unwrap();
    println!("¿libre? {}", m.try_lock().is_ok());
}
