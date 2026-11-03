use std::ops::Deref;

struct Nombre(String);

impl Deref for Nombre {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

fn saluda(n: &str) {
    println!("Hola, {n}");
}

fn main() {
    let n = Nombre(String::from("Lucía"));
    saluda(&n); // &Nombre → &String → &str
    println!("{} letras", n.chars().count());
}
