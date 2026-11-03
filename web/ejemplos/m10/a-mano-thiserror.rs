use std::fmt;

#[derive(Debug)]
enum ErrorTexto {
    Vacio,
    Larga(usize),
}

impl fmt::Display for ErrorTexto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorTexto::Vacio => write!(f, "archivo vacío"),
            ErrorTexto::Larga(n) => write!(f, "línea {n} demasiado larga"),
        }
    }
}

impl std::error::Error for ErrorTexto {}

fn main() {
    let errores: Vec<Box<dyn std::error::Error>> = vec![Box::new(ErrorTexto::Vacio), Box::new(ErrorTexto::Larga(7))];
    for e in errores {
        println!("{e}");
    }
}
