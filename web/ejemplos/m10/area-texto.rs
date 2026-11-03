use std::num::ParseIntError;

fn area(ancho: &str, alto: &str) -> Result<u32, ParseIntError> {
    let a: u32 = ancho.parse()?;
    let b: u32 = alto.parse()?;
    Ok(a * b)
}

fn main() {
    println!("{:?}", area("6", "7"));
    println!("{:?}", area("6", "siete"));
}
