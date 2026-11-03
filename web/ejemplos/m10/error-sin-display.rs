#[derive(Debug)]
enum MiError {
    Vacio,
}

impl std::error::Error for MiError {}

fn main() {
    println!("{:?}", MiError::Vacio);
}
