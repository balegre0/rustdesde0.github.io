use std::num::ParseIntError;

fn edad(texto: &str) -> Result<u8, ParseIntError> {
    texto.trim().parse::<u8>()
}

// main puede devolver Result: si es Err, Rust imprime el error y sale con código 1
fn main() -> Result<(), ParseIntError> {
    println!("edad = {}", edad("41")?);
    println!("edad = {}", edad("cuatrocientos")?);
    println!("esta línea no se imprime");
    Ok(())
}
