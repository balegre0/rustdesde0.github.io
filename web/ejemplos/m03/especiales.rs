fn main() {
    // 1. Dividir un decimal entre cero NO es un pánico
    let inf = 1.0 / 0.0;
    println!("1.0 / 0.0 = {inf}");
    println!("-1.0 / 0.0 = {}", -1.0 / 0.0);
    // 2. NaN: no es un número
    let nan = 0.0f64 / 0.0;
    println!("0.0 / 0.0 = {nan}");
    // 3. NaN no es igual ni a sí mismo
    println!("nan == nan: {}", nan == nan);
    println!("nan.is_nan(): {}", nan.is_nan());
    // 4. Existen dos ceros
    println!("-0.0 == 0.0: {}", -0.0f64 == 0.0);
    // 5. Comparar decimales con un margen
    let suma: f64 = 0.1 + 0.2;
    println!("casi iguales: {}", (suma - 0.3).abs() < 1e-9);
}
