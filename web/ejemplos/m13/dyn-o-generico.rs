trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);
struct Circulo(f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}
impl Forma for Circulo {
    fn area(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

// Genérico: una copia por tipo; todos los elementos del slice son del MISMO tipo.
fn suma_generica<T: Forma>(formas: &[T]) -> f64 {
    formas.iter().map(|f| f.area()).sum()
}

// dyn: una sola copia; cada elemento puede ser de un tipo distinto.
fn suma_dyn(formas: &[&dyn Forma]) -> f64 {
    formas.iter().map(|f| f.area()).sum()
}

fn main() {
    println!("{}", suma_generica(&[Cuadrado(1.0), Cuadrado(2.0)]));
    println!("{}", suma_dyn(&[&Cuadrado(1.0), &Circulo(1.0)]));
}
