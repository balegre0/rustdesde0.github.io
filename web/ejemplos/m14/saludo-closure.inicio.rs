fn main() {
    let exclamaciones = 3;
    // Escribe un closure `grita` que reciba un &str y devuelva un String
    // con el texto en mayúsculas seguido de tantos '!' como `exclamaciones`.
    // Pista: "hola".to_uppercase() y "!".repeat(n).
    let grita = |texto: &str| -> String { todo!() };
    println!("{}", grita("hola"));
    println!("{}", grita("ñandú"));
}
