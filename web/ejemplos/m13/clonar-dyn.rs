trait Figura: Clone {
    fn lados(&self) -> u32;
}

#[derive(Clone)]
struct Triangulo;

impl Figura for Triangulo {
    fn lados(&self) -> u32 {
        3
    }
}

fn main() {
    let figuras: Vec<Box<dyn Figura>> = vec![Box::new(Triangulo)];
    println!("{}", figuras[0].lados());
}
