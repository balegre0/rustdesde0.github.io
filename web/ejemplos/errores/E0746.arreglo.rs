trait Animal {
    fn sonido(&self) -> String;
}

struct Perro;

impl Animal for Perro {
    fn sonido(&self) -> String {
        String::from("guau")
    }
}

fn crear() -> Box<dyn Animal> {
    Box::new(Perro)
}

fn main() {
    println!("{}", crear().sonido());
}
