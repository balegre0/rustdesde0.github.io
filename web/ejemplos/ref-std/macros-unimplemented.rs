trait Figura {
    fn area(&self) -> f64;
    fn perimetro(&self) -> f64;
}

struct Cuadrado(f64);

impl Figura for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
    fn perimetro(&self) -> f64 {
        // No vamos a hacerlo (todavía, o nunca) para este tipo.
        unimplemented!("perímetro de {}", "Cuadrado")
    }
}

fn main() {
    let c = Cuadrado(3.0);
    println!("área = {}", c.area());
    println!("perímetro = {}", c.perimetro());
}
