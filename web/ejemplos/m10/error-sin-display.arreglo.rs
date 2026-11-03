use std::fmt;

#[derive(Debug)]
enum MiError {
    Vacio,
}

impl fmt::Display for MiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MiError::Vacio => write!(f, "la entrada está vacía"),
        }
    }
}

impl std::error::Error for MiError {}

fn main() {
    println!("{:?} / {}", MiError::Vacio, MiError::Vacio);
}
