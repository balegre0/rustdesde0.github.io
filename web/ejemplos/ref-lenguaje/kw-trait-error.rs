trait Saludar {
    fn hola(&self) -> String;
    fn adios(&self) -> String;
}

struct Robot;

// Falta implementar `adios`, que no tiene versión por defecto.
impl Saludar for Robot {
    fn hola(&self) -> String { "bip".into() }
}

fn main() {}
