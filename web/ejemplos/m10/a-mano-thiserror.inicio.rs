// Escribe a mano lo que generaría thiserror para este enum:
//   #[error("archivo vacío")]                Vacio
//   #[error("línea {0} demasiado larga")]    Larga(usize)
// Solo hace falta el Display y el `impl Error` vacío.

use std::fmt;

#[derive(Debug)]
enum ErrorTexto {
    Vacio,
    Larga(usize),
}

// escribe aquí impl fmt::Display e impl std::error::Error

fn main() {
    let errores: Vec<Box<dyn std::error::Error>> = vec![Box::new(ErrorTexto::Vacio), Box::new(ErrorTexto::Larga(7))];
    for e in errores {
        println!("{e}");
    }
}
