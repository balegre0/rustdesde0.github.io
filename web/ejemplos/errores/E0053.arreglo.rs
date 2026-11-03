trait Escalar {
    fn escalar(&self, factor: f64) -> f64;
}

struct Medida(f64);

impl Escalar for Medida {
    fn escalar(&self, factor: f64) -> f64 {
        self.0 * factor
    }
}

fn main() {
    println!("{}", Medida(1.5).escalar(2.0));
}
