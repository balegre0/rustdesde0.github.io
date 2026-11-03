use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorRango {
    valor: i64,
}

impl fmt::Display for ErrorRango {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} está fuera de 0..=100", self.valor)
    }
}

impl Error for ErrorRango {}

// 1. Box<dyn Error>: «cualquier error». `?` mete en la caja errores de cualquier tipo.
fn leer_porcentaje(texto: &str) -> Result<u8, Box<dyn Error>> {
    if texto.is_empty() {
        return Err("texto vacío".into()); // un &str también se convierte
    }
    let n: i64 = texto.parse()?; // ParseIntError -> Box<dyn Error>
    if !(0..=100).contains(&n) {
        return Err(Box::new(ErrorRango { valor: n })); // nuestro propio error
    }
    Ok(n as u8)
}

fn main() {
    for t in ["42", "", "abc", "250"] {
        match leer_porcentaje(t) {
            Ok(p) => println!("{t:?}: {p}%"),
            Err(e) => {
                println!("{t:?}: error: {e}");
                // 2. downcast_ref: preguntar si dentro de la caja hay un tipo concreto.
                if let Some(r) = e.downcast_ref::<ErrorRango>() {
                    println!("      (es un ErrorRango con valor {})", r.valor);
                }
            }
        }
    }
    println!("tamaño de Box<dyn Error>: {} bytes", std::mem::size_of::<Box<dyn Error>>());
}
