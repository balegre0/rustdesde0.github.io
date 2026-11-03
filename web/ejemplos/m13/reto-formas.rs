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

impl Forma for Rect {
    fn area(&self) -> f64 {
        self.0 * self.1
    }
}

fn area_total(formas: &[Box<dyn Forma>]) -> f64 {
    formas.iter().map(|f| f.area()).sum()
}

fn main() {
    let formas: Vec<Box<dyn Forma>> = vec![Box::new(Cuadrado(2.0)), Box::new(Rect(1.0, 3.0)), Box::new(Cuadrado(0.5))];
    println!("{}", area_total(&formas));
}
