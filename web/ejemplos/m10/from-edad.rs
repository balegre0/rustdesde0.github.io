use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorEdad {
    NoNumero,
    Imposible(u8),
}

impl From<ParseIntError> for ErrorEdad {
    fn from(_: ParseIntError) -> Self {
        ErrorEdad::NoNumero
    }
}

fn leer_edad(texto: &str) -> Result<u8, ErrorEdad> {
    let n = texto.trim().parse::<u8>()?;
    if n > 130 {
        return Err(ErrorEdad::Imposible(n));
    }
    Ok(n)
}

fn main() {
    println!("{:?}", leer_edad("33"));
    println!("{:?}", leer_edad("200"));
    println!("{:?}", leer_edad("treinta"));
}
