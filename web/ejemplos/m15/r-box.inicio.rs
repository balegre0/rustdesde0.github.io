fn main() {
    // 1. Guarda 2.5 en el heap con Box::new.
    let precio: Box<f64> = todo!();
    // 2. Desreferencia con * para calcular el doble.
    let doble: f64 = todo!();
    println!("doble = {doble}");
    let _ = precio;
}
