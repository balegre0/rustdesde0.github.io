// Completa el método `perimetro` (2 · π · radio) y el método `escalar`,
// que multiplica el radio por `factor`.

struct Circulo {
    radio: f64,
}

impl Circulo {
    fn perimetro(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radio
    }
    fn escalar(&mut self, factor: f64) {
        self.radio *= factor;
    }
}

fn main() {
    let mut c = Circulo { radio: 1.0 };
    println!("{:.2}", c.perimetro());
    c.escalar(3.0);
    println!("{:.2}", c.perimetro());
}
