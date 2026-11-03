use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct ErrorConfig {
    clave: String,
    causa: ParseIntError,
}

impl fmt::Display for ErrorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "valor inválido para «{}»", self.clave)
    }
}

impl Error for ErrorConfig {
    // 1. source: «¿qué error causó este?». Forma una cadena de causas.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.causa)
    }
}

fn leer_puerto(valor: &str) -> Result<u16, ErrorConfig> {
    valor.parse().map_err(|causa| ErrorConfig { clave: String::from("puerto"), causa })
}

fn main() {
    let Err(e) = leer_puerto("80800") else { return };
    // 2. Recorrer la cadena: el error y cada una de sus causas.
    println!("error: {e}");
    let mut actual = e.source();
    while let Some(causa) = actual {
        println!("  causado por: {causa}");
        actual = causa.source();
    }
}
