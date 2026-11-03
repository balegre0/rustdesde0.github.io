trait Animal {
    fn sonido(&self) -> &'static str;
}

struct Perro;
struct Gato;

impl Animal for Perro {
    fn sonido(&self) -> &'static str {
        "guau"
    }
}
impl Animal for Gato {
    fn sonido(&self) -> &'static str {
        "miau"
    }
}

fn main() {
    let animales: Vec<Box<dyn Animal>> = vec![Box::new(Perro), Box::new(Gato)];
    for a in &animales {
        println!("{}", a.sonido());
    }
}
