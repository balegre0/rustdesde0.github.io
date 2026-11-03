// Cambia la firma de `doble` para que devuelva Result<i32, Box<dyn Error>>,
// así los DOS `?` (de ParseIntError y del &str) compilan.

use std::error::Error;

fn doble(texto: &str) -> Result<i32, std::num::ParseIntError> {
    if texto.is_empty() {
        Err("vacío")?;
    }
    let n: i32 = texto.parse()?;
    Ok(n * 2)
}

fn main() {
    for t in ["21", "", "x"] {
        match doble(t) {
            Ok(v) => println!("{v}"),
            Err(e) => println!("error: {e}"),
        }
    }
}
