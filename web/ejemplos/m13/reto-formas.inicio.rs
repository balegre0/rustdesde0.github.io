trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);
struct Rect(f64, f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

// 1. Implementa Forma para Rect (área = ancho * alto).
// 2. Escribe `area_total(formas: &[Box<dyn Forma>]) -> f64`.
// 3. En main, crea un Vec<Box<dyn Forma>> con Cuadrado(2.0), Rect(1.0, 3.0)
//    y Cuadrado(0.5), e imprime su área total.

fn main() {
    println!("{}", Cuadrado(2.0).area());
}
