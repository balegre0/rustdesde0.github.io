// Una lista enlazada: el Box permite que el tipo se contenga a sí mismo
#[derive(Debug)]
enum Lista {
    Nodo(i32, Box<Lista>),
    Fin,
}

fn suma(l: &Lista) -> i32 {
    match l {
        Lista::Nodo(v, resto) => v + suma(resto),
        Lista::Fin => 0,
    }
}

trait Figura {
    fn area(&self) -> f64;
}
struct Cuadrado(f64);
struct Circulo(f64);
impl Figura for Cuadrado {
    fn area(&self) -> f64 { self.0 * self.0 }
}
impl Figura for Circulo {
    fn area(&self) -> f64 { 3.14 * self.0 * self.0 }
}

fn main() {
    // 1. un valor en el montón
    let b = Box::new(41);
    println!("{}", *b + 1);
    // 2. tipos recursivos
    let l = Lista::Nodo(1, Box::new(Lista::Nodo(2, Box::new(Lista::Fin))));
    println!("{l:?} suma {}", suma(&l));
    // 3. objetos de trait de tipos distintos en un mismo Vec
    let figuras: Vec<Box<dyn Figura>> = vec![Box::new(Cuadrado(2.0)), Box::new(Circulo(1.0))];
    for f in &figuras {
        println!("{}", f.area());
    }
    // 4. un Box mide lo que un puntero
    println!("{} bytes", std::mem::size_of::<Box<[u8; 1000]>>());
}
