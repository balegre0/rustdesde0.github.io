use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct SinPermiso {
    usuario: String,
}

impl fmt::Display for SinPermiso {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} no tiene permiso", self.usuario)
    }
}

impl Error for SinPermiso {}

fn main() -> Result<(), Box<dyn Error>> {
    println!("comprobando…");
    Err(Box::new(SinPermiso { usuario: String::from("marta") }))
}
