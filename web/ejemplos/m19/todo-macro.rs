#[allow(dead_code)]
enum Forma { Circulo(f64), Cuadrado(f64) }

fn area(f: &Forma) -> f64 {
    match f {
        Forma::Cuadrado(l) => l * l,
        // 1. todo!() compila con cualquier tipo, pero entra en pánico al ejecutarse
        Forma::Circulo(_) => todo!("falta el área del círculo"),
    }
}

fn main() {
    println!("{}", area(&Forma::Cuadrado(3.0)));
    println!("{}", area(&Forma::Circulo(1.0)));
}
