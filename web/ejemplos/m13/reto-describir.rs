trait Animal {
    fn nombre(&self) -> String;
    fn sonido(&self) -> String;

    fn presentarse(&self) -> String {
        format!("{} dice {}", self.nombre(), self.sonido())
    }
}

struct Perro;
struct Vaca;

impl Animal for Perro {
    fn nombre(&self) -> String {
        String::from("El perro")
    }
    fn sonido(&self) -> String {
        String::from("guau")
    }
}

impl Animal for Vaca {
    fn nombre(&self) -> String {
        String::from("La vaca")
    }
    fn sonido(&self) -> String {
        String::from("muu")
    }
}

fn main() {
    println!("{}", Perro.presentarse());
    println!("{}", Vaca.presentarse());
}
