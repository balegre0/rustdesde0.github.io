fn main() {
    let exclamaciones = 3;
    let grita = |texto: &str| -> String { texto.to_uppercase() + &"!".repeat(exclamaciones) };
    println!("{}", grita("hola"));
    println!("{}", grita("ñandú"));
}
