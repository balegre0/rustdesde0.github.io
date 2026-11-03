use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorConfig {
    linea: usize,
    causa: std::num::ParseIntError,
}

impl fmt::Display for ErrorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuración mal escrita en la línea {}", self.linea)
    }
}

impl Error for ErrorConfig {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.causa)
    }
}

fn leer_puerto(texto: &str) -> Result<u16, ErrorConfig> {
    texto.trim().parse().map_err(|causa| ErrorConfig { linea: 3, causa })
}

fn main() {
    if let Err(e) = leer_puerto("80a") {
        println!("error: {e}");
        // Recorrer la cadena de causas.
        let mut causa = e.source();
        while let Some(c) = causa {
            println!("  causado por: {c}");
            causa = c.source();
        }
        let caja: Box<dyn Error> = Box::new(e);
        println!("en una caja: {caja}");
    }
}
