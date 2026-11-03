fn main() {
    // 1. Un literal con punto es f64 por defecto
    let pi = 3.14159;
    let e: f32 = 2.71828;
    println!("pi = {pi}, e = {e}");
    // 2. Elegir cuántos decimales mostrar
    println!("pi con 2 decimales: {pi:.2}");
    // 3. La división entre decimales sí da decimales
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);
    println!("7 / 2     = {}", 7 / 2);
    // 4. La sorpresa de siempre
    println!("0.1 + 0.2 = {}", 0.1 + 0.2);
    println!("es 0.3: {}", 0.1 + 0.2 == 0.3);
    // 5. f32 tiene menos precisión que f64
    println!("f32: {}", 0.1f32 + 0.2f32);
}
