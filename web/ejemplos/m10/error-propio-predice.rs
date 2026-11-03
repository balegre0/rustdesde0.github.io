use std::fmt;

#[derive(Debug)]
enum ErrorEdad {
    NoEsNumero,
    Imposible(u32),
}

impl fmt::Display for ErrorEdad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorEdad::NoEsNumero => write!(f, "eso no es un número"),
            ErrorEdad::Imposible(n) => write!(f, "nadie tiene {n} años"),
        }
    }
}

fn main() {
    let e = ErrorEdad::Imposible(200);
    println!("{}", e);
    println!("{:?}", e);
}
