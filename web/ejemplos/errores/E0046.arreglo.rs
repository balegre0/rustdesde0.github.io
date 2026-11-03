trait Figura {
    fn area(&self) -> f64;
    fn nombre(&self) -> String;
}

struct Cuadrado(f64);

impl Figura for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
    fn nombre(&self) -> String {
        String::from("cuadrado")
    }
}

fn main() {
    let c = Cuadrado(2.0);
    println!("{} {}", c.nombre(), c.area());
}
