fn main() {
    match "12".parse::<u32>() {
        Ok(precio) => println!("{}", precio * 121 / 100),
        Err(e) => println!("precio inválido: {e}"),
    }
}
