use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

// 1. Un error propio que ENVUELVE errores de otros tipos.
#[derive(Debug)]
enum ErrorProducto {
    Unidades(ParseIntError),
    Precio(ParseFloatError),
    Formato(String),
}

impl fmt::Display for ErrorProducto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorProducto::Unidades(e) => write!(f, "unidades inválidas: {e}"),
            ErrorProducto::Precio(e) => write!(f, "precio inválido: {e}"),
            ErrorProducto::Formato(linea) => write!(f, "línea mal formada: «{linea}»"),
        }
    }
}

impl std::error::Error for ErrorProducto {}

// 2. Una conversión From por cada error envuelto: es lo que usa `?`.
impl From<ParseIntError> for ErrorProducto {
    fn from(e: ParseIntError) -> Self {
        ErrorProducto::Unidades(e)
    }
}

impl From<ParseFloatError> for ErrorProducto {
    fn from(e: ParseFloatError) -> Self {
        ErrorProducto::Precio(e)
    }
}

// 3. Ahora `?` convierte cada error al tipo de la función, sin map_err.
fn total(linea: &str) -> Result<f64, ErrorProducto> {
    let Some((unidades, precio)) = linea.split_once('x') else {
        return Err(ErrorProducto::Formato(linea.to_string()));
    };
    let u: u32 = unidades.trim().parse()?; // ParseIntError  -> ErrorProducto
    let p: f64 = precio.trim().parse()?;   // ParseFloatError -> ErrorProducto
    Ok(u as f64 * p)
}

fn main() {
    for linea in ["3 x 2.50", "tres x 2.50", "3 x dos", "3 por 2.50"] {
        match total(linea) {
            Ok(t) => println!("{linea:>12} → {t:.2} €"),
            Err(e) => println!("{linea:>12} → error: {e}"),
        }
    }
}
