fn main() {
    // 1. Conversión numérica: trunca los bits que no caben.
    let grande: i32 = 300;
    println!("{}", grande as u8);
    // 2. De flotante a entero: satura y quita decimales.
    println!("{}", 3.99_f64 as i32);
    println!("{}", -1.0_f64 as u8);
    println!("{}", 1e10_f64 as i32);
    // 3. De u8 a char.
    println!("{}", 65u8 as char);
    // 4. Renombrar al importar.
    use std::collections::HashMap as Mapa;
    let mut m = Mapa::new();
    m.insert("uno", 1);
    println!("{:?}", m.get("uno"));
}
