use std::num::ParseIntError;

fn sumar_textos(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    println!("  primero leído: {x}");
    let y: i32 = b.parse()?;
    println!("  segundo leído: {y}");
    Ok(x + y)
}

fn main() {
    println!("{:?}", sumar_textos("4", "5"));
    println!("{:?}", sumar_textos("4", "cinco"));
}
