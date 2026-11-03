use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum ErrorPunto {
    SinComa,
    Coordenada { eje: char, causa: ParseIntError },
}

impl fmt::Display for ErrorPunto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ErrorPunto {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}

fn leer_punto(texto: &str) -> Result<Punto, ErrorPunto> {
    todo!()
}

fn distancia2(a: &str, b: &str) -> Result<i64, Box<dyn Error>> {
    todo!()
}
