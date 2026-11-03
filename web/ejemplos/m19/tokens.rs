fn main() {
    // 1. Una macro recibe tokens, no valores: stringify! los devuelve como texto
    println!("{}", stringify!(let x = 5;));
    // 2. Los espacios y saltos de línea no importan: solo cuentan los tokens
    println!("{}", stringify!(
        a   +
            b
    ));
    // 3. Ni siquiera tiene que ser Rust válido, basta con que sean tokens
    println!("{}", stringify!(hola mundo ñandú 42));
}
