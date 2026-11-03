use std::fmt::Display;

// Con Box<dyn Display> cada rama puede devolver un tipo distinto:
// la elección se hace al ejecutar, no al compilar.
fn etiqueta(numero: bool) -> Box<dyn Display> {
    if numero {
        Box::new(42)
    } else {
        Box::new("cuarenta y dos")
    }
}

fn main() {
    println!("{}", etiqueta(true));
    println!("{}", etiqueta(false));
}
