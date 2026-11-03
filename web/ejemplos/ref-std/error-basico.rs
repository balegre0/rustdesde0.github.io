use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorEdad {
    NoEsNumero(ParseIntError),
    FueraDeRango(u32),
}

impl fmt::Display for ErrorEdad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorEdad::NoEsNumero(_) => write!(f, "la edad no es un número"),
            ErrorEdad::FueraDeRango(n) => write!(f, "{n} no es una edad válida"),
        }
    }
}

impl Error for ErrorEdad {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorEdad::NoEsNumero(e) => Some(e), // la causa original
            ErrorEdad::FueraDeRango(_) => None,
        }
    }
}

// From permite que `?` convierta el error solo
impl From<ParseIntError> for ErrorEdad {
    fn from(e: ParseIntError) -> Self {
        ErrorEdad::NoEsNumero(e)
    }
}

fn leer_edad(t: &str) -> Result<u32, ErrorEdad> {
    let n: u32 = t.parse()?;
    if n > 150 {
        return Err(ErrorEdad::FueraDeRango(n));
    }
    Ok(n)
}

// Box<dyn Error> acepta cualquier error
fn principal() -> Result<(), Box<dyn Error>> {
    println!("{}", leer_edad("30")?);
    let n: i32 = "x".parse()?;
    println!("{n}");
    Ok(())
}

fn main() {
    for t in ["30", "200", "treinta"] {
        match leer_edad(t) {
            Ok(n) => println!("{t}: {n}"),
            Err(e) => println!("{t}: {e} (causa: {:?})", e.source().map(|c| c.to_string())),
        }
    }
    if let Err(e) = principal() {
        println!("principal falló: {e}");
    }
}
