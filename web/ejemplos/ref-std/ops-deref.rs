use std::ops::{Deref, DerefMut};

struct Nombre(String);

impl Deref for Nombre {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl DerefMut for Nombre {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

fn saludar(s: &str) {
    println!("hola, {s}");
}

fn main() {
    let mut n = Nombre(String::from("Ana"));
    // métodos de String a través de Deref
    println!("{} {}", n.len(), n.to_uppercase());
    // coerción automática: &Nombre -> &String -> &str
    saludar(&n);
    n.push_str(" Gil"); // DerefMut
    println!("{}", *n);
    // Box, String y Vec también usan Deref
    let b = Box::new(5);
    println!("{}", *b + 1);
}
