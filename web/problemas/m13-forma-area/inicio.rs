trait Figura {
    fn area(&self) -> f64;
    fn nombre(&self) -> String {
        todo!()
    }
}

struct Rectangulo {
    ancho: f64,
    alto: f64,
}

struct Circulo {
    radio: f64,
}

impl Figura for Rectangulo {
    fn area(&self) -> f64 {
        todo!()
    }
}

impl Figura for Circulo {
    fn area(&self) -> f64 {
        todo!()
    }
}

fn area_total(figuras: &[&dyn Figura]) -> f64 {
    todo!()
}
