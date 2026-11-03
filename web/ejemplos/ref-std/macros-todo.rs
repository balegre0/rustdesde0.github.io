#[allow(dead_code)]
trait Forma {
    fn area(&self) -> f64;
    fn perimetro(&self) -> f64;
}

struct Cuadrado(f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
    fn perimetro(&self) -> f64 {
        todo!("falta calcular el perímetro") // compila, pero entra en pánico si se llama
    }
}

fn main() {
    let c = Cuadrado(3.0);
    println!("{}", c.area());
    println!("{}", c.perimetro());
}
