// 1. Un enum sin datos: elegir uno entre varios casos (variantes).
#[derive(Debug)]
enum Palo {
    Oros,
    Copas,
    Espadas,
    Bastos,
}

// 2. Variantes con datos: cada una lleva lo que necesita, y nada más.
#[derive(Debug)]
enum Forma {
    Circulo { radio: f64 }, // con campos con nombre
    Rectangulo(f64, f64),   // como un tuple struct
    Punto,                  // sin datos
}

impl Forma {
    // 3. Los enums también tienen métodos.
    fn area(&self) -> f64 {
        match self {
            Forma::Circulo { radio } => 3.14159 * radio * radio,
            Forma::Rectangulo(ancho, alto) => ancho * alto,
            Forma::Punto => 0.0,
        }
    }
}

fn main() {
    let p = Palo::Copas;
    println!("palo: {:?}", p);

    // 4. Un Vec puede mezclar variantes: todas son del mismo tipo, Forma.
    let formas = vec![Forma::Circulo { radio: 1.0 }, Forma::Rectangulo(2.0, 3.0), Forma::Punto];
    for f in &formas {
        println!("{:?} → área {:.2}", f, f.area());
    }

    // 5. `use` evita repetir el nombre del enum.
    use Palo::*;
    let mano = [Oros, Espadas, Bastos];
    println!("mano: {:?}", mano);
}
