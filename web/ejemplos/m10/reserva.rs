fn leer_volumen(texto: &str) -> u8 {
    texto.parse().unwrap_or(50)
}

fn main() {
    println!("{}", leer_volumen("80"));
    println!("{}", leer_volumen("muy alto"));
    println!("{}", leer_volumen("999"));
}
