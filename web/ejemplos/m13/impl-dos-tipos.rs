use std::fmt::Display;

fn etiqueta(numero: bool) -> impl Display {
    if numero {
        42
    } else {
        "cuarenta y dos"
    }
}

fn main() {
    println!("{}", etiqueta(true));
}
