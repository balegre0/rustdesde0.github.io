use std::sync::RwLock;

fn main() {
    let precios = RwLock::new(vec![10, 20, 30]);
    // 1. Con un lector, suma los precios.
    let suma: i32 = 0;
    println!("suma = {suma}");
    // 2. Con un escritor, duplica cada precio.
    let _ = &precios;
    println!("{:?}", precios.read().unwrap());
}
