use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let ancho: u32 = "12".parse()?;
    println!("ancho = {ancho}");
    let alto: u32 = "doce".parse()?;
    println!("alto = {alto}");
    Ok(())
}
