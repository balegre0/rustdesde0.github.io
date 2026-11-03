enum Forma {
    Circulo { radio: f64 },
    Cuadrado { lado: f64 },
}

fn main() {
    let formas = [Forma::Circulo { radio: 2.0 }, Forma::Cuadrado { lado: 3.0 }];
    for f in formas {
        match f {
            Forma::Circulo { radio } => println!("radio = {radio}"),
            Forma::Cuadrado { lado } => println!("lado = {lado}"),
        }
    }
}
