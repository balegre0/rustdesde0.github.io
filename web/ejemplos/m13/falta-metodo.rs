trait Forma {
    fn area(&self) -> f64;
    fn nombre(&self) -> String;
}

struct Cuadrado {
    lado: f64,
}

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.lado * self.lado
    }
}

fn main() {
    let c = Cuadrado { lado: 2.0 };
    println!("{}", c.area());
}
