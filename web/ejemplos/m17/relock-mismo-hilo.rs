use std::sync::Mutex;

fn main() {
    let m = Mutex::new(0);
    // 1. Tomamos el candado y guardamos el guard en `_a`: sigue vivo.
    let _a = m.lock().unwrap();
    println!("primer lock: ok");
    // 2. El MISMO hilo intenta tomarlo otra vez sin haberlo soltado.
    let _b = m.lock().unwrap();
    println!("segundo lock: ok");
}
