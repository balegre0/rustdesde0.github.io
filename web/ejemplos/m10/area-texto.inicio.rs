// Completa `area` con el operador ?: convierte ancho y alto (texto) a u32
// y devuelve su producto. Si alguno no es un número, devuelve el error de parse.

use std::num::ParseIntError;

fn area(ancho: &str, alto: &str) -> Result<u32, ParseIntError> {
    todo!()
}

fn main() {
    println!("{:?}", area("6", "7"));
    println!("{:?}", area("6", "siete"));
}
