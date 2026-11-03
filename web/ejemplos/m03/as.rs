fn main() {
    // 1. De grande a pequeño: se quedan los bits de abajo (trunca)
    println!("300 as u8 = {}", 300i32 as u8);
    println!("256 as u8 = {}", 256i32 as u8);
    // 2. Con signo a sin signo: los mismos bits, otra lectura
    println!("-1 as u32 = {}", -1i32 as u32);
    println!("200 as i8 = {}", 200u8 as i8);
    // 3. De decimal a entero: corta los decimales y SATURA
    println!("3.99 as u8 = {}", 3.99f64 as u8);
    println!("-5.0 as u8 = {}", -5.0f64 as u8);
    println!("1e10 as u8 = {}", 1e10f64 as u8);
    println!("NaN as u8 = {}", f64::NAN as u8);
    // 4. Caracteres y números
    println!("'A' as u32 = {}", 'A' as u32);
    println!("'ñ' as u32 = {}", 'ñ' as u32);
    println!("97u8 as char = {}", 97u8 as char);
    // 5. De pequeño a grande: siempre exacto
    println!("255u8 as u64 = {}", 255u8 as u64);
}
