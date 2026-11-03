use std::f64::consts::PI;

enum Forma {
    Circulo { radio: f64 },
    Rectangulo { ancho: f64, alto: f64 },
    Triangulo { base: f64, altura: f64 },
}

fn area(f: &Forma) -> f64 {
    match f {
        Forma::Circulo { radio } => PI * radio * radio,
        Forma::Rectangulo { ancho, alto } => ancho * alto,
        Forma::Triangulo { base, altura } => base * altura / 2.0,
    }
}
