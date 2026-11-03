fn main() {
    // 1. Un literal entre comillas dobles es un &str:
    //    texto fijo, guardado dentro del propio programa
    let saludo: &str = "hola";

    // 2. Un String es texto propio que puede crecer
    let mut nombre = String::from("Begoña");
    nombre.push_str(" Ibáñez");
    nombre.push('!');
    println!("{saludo}, {nombre}");

    // 3. len() cuenta BYTES, no letras
    let palabra = "año";
    println!("\"{palabra}\": {} bytes, {} caracteres", palabra.len(), palabra.chars().count());

    // 4. Los bytes de verdad (UTF-8)
    println!("bytes: {:?}", palabra.as_bytes());

    // 5. Recorrer los caracteres uno a uno
    for c in palabra.chars() {
        print!("[{c}] ");
    }
    println!();
}
