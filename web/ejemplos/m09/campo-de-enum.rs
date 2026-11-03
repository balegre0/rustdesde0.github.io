enum Forma {
    Circulo { radio: f64 },
    Cuadrado { lado: f64 },
}

fn main() {
    let f = Forma::Circulo { radio: 2.0 };
    println!("{}", f.radio);
}
