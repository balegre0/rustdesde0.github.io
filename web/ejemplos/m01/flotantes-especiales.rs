// Valores especiales de la coma flotante.
fn main() {
    // 1. Dividir entre cero no detiene el programa: da infinito
    let infinito = 1.0f64 / 0.0;
    println!("1 / 0     = {infinito}");

    // 2. Hay dos ceros: +0 y -0 (solo cambia el bit de signo)
    println!("+0 bits   = {:#018x}", 0.0f64.to_bits());
    println!("-0 bits   = {:#018x}", (-0.0f64).to_bits());
    println!("+0 == -0  : {}", 0.0f64 == -0.0);

    // 3. NaN («no es un número»): ni siquiera es igual a sí mismo
    let nan = 0.0f64 / 0.0;
    println!("0 / 0     = {nan}");
    println!("NaN == NaN: {}", nan == nan);
    println!("es NaN    : {}", nan.is_nan());
}
