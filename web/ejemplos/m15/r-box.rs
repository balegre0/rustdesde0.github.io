fn main() {
    // 1. Guarda 2.5 en el heap con Box::new.
    let precio: Box<f64> = Box::new(2.5);
    // 2. Desreferencia con * para calcular el doble.
    let doble = *precio * 2.0;
    println!("doble = {doble}");
}
