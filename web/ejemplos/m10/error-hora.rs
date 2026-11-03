use std::fmt;

#[derive(Debug)]
enum ErrorHora {
    FueraDeRango(u32),
    Formato,
}

impl fmt::Display for ErrorHora {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorHora::FueraDeRango(h) => write!(f, "la hora {h} no existe"),
            ErrorHora::Formato => write!(f, "usa el formato HH:MM"),
        }
    }
}

impl std::error::Error for ErrorHora {}

fn main() {
    println!("{}", ErrorHora::FueraDeRango(25));
    println!("{}", ErrorHora::Formato);
}
