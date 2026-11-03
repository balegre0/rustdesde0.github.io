trait Forma {
    fn area(&self) -> f64;
}

struct Cuadrado(f64);
struct Circulo(f64);

impl Forma for Cuadrado {
    fn area(&self) -> f64 { self.0 * self.0 }
}
impl Forma for Circulo {
    fn area(&self) -> f64 { 3.0 * self.0 * self.0 }
}

fn main() {
    // 1. `dyn Forma`: «algún tipo que implementa Forma», decidido al ejecutar.
    let formas: Vec<Box<dyn Forma>> = vec![Box::new(Cuadrado(2.0)), Box::new(Circulo(1.0))];
    for f in &formas {
        println!("{}", f.area());
    }
    // 2. `&dyn Forma` es un puntero ancho: dirección de los datos + tabla de métodos.
    println!("{} punteros", size_of::<&dyn Forma>() / size_of::<usize>());
}
