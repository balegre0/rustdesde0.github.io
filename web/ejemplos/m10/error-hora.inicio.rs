// Completa el Display de ErrorHora:
//   FueraDeRango(h) -> "la hora h no existe"
//   Formato         -> "usa el formato HH:MM"

use std::fmt;

#[derive(Debug)]
enum ErrorHora {
    FueraDeRango(u32),
    Formato,
}

impl fmt::Display for ErrorHora {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for ErrorHora {}

fn main() {
    println!("{}", ErrorHora::FueraDeRango(25));
    println!("{}", ErrorHora::Formato);
}
