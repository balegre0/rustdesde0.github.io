trait Escalar {
    fn escalar(&self, factor: f64) -> f64;
}

struct Medida(f64);

impl Escalar for Medida {
    fn escalar(&self, factor: i32) -> f64 {
        self.0 * factor as f64
    }
}

fn main() {
    println!("{}", Medida(1.5).escalar(2.0));
}
