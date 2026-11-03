struct Contador { n: u32 }

impl Contador {
    fn consumir(self) -> u32 { self.n }
}

fn main() {
    let c = Contador { n: 3 };
    // Un método con `self` (sin &) se queda con el valor: después ya no existe.
    println!("{}", c.consumir());
    println!("{}", c.n);
}
