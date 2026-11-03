trait Figura {
    fn area(&self) -> f64;
    fn nombre(&self) -> String {
        String::from("figura")
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
        self.ancho * self.alto
    }
    fn nombre(&self) -> String {
        String::from("rectángulo")
    }
}

impl Figura for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radio * self.radio
    }
}

fn area_total(figuras: &[&dyn Figura]) -> f64 {
    figuras.iter().map(|f| f.area()).sum()
}
