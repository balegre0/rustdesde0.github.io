use std::fmt::Display;

fn mostrar<T>(x: T) -> String
where
    T: Display,
{
    format!("{x}")
}

struct Secreto;

fn main() {
    // `Secreto` no cumple el límite `T: Display`.
    println!("{}", mostrar(Secreto));
}
