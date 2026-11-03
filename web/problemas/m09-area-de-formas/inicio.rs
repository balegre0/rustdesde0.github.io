use std::f64::consts::PI;

enum Forma {
    Circulo { radio: f64 },
    Rectangulo { ancho: f64, alto: f64 },
    Triangulo { base: f64, altura: f64 },
}

fn area(f: &Forma) -> f64 {
    todo!()
}
