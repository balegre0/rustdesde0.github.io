trait Contenedor {
    type Item;
    const CAPACIDAD: usize;

    fn primero(&self) -> Option<&Self::Item>;
}

struct Caja {
    cosas: Vec<String>,
}

impl Contenedor for Caja {
    type Item = String;
    const CAPACIDAD: usize = 3;

    fn primero(&self) -> Option<&String> {
        self.cosas.first()
    }
}

fn main() {
    let c = Caja { cosas: vec![String::from("llave"), String::from("moneda")] };
    println!("{:?} {}", c.primero(), Caja::CAPACIDAD);
}
