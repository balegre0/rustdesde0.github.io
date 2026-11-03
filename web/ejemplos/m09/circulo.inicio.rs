// Completa el método `perimetro` (2 · π · radio) y el método `escalar`,
// que multiplica el radio por `factor`.
// El programa debe imprimir:
// 6.28
// 18.85

struct Circulo {
    radio: f64,
}

impl Circulo {
    fn perimetro(&self) -> f64 {
        0.0 // cámbialo
    }
    // escribe aquí `escalar`
}

fn main() {
    let mut c = Circulo { radio: 1.0 };
    println!("{:.2}", c.perimetro());
    // c.escalar(3.0);
    println!("{:.2}", c.perimetro());
}
