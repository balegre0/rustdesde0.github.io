struct Cuadrado { lado: f64 }
struct Rectangulo { ancho: f64, alto: f64 }
struct Circulo { radio: f64 }

impl_area! {
    Cuadrado, c => c.lado * c.lado;
    Rectangulo, r => r.ancho * r.alto;
    Circulo, yo => 3.0 * yo.radio * yo.radio;
}

fn main() {
    let mut ok = true;
    let mut caso = |nombre: &str, obtenido: f64, esperado: f64| {
        if (obtenido - esperado).abs() < 1e-9 { println!("OK: {}", nombre); }
        else { println!("FALLO: {} (esperado {}, obtenido {})", nombre, esperado, obtenido); ok = false; }
    };
    caso("cuadrado", Cuadrado { lado: 3.0 }.area(), 9.0);
    caso("rectángulo", Rectangulo { ancho: 2.0, alto: 5.0 }.area(), 10.0);
    caso("círculo (pi = 3)", Circulo { radio: 2.0 }.area(), 12.0);
    let figuras: Vec<Box<dyn Area>> = vec![Box::new(Cuadrado { lado: 1.0 }), Box::new(Circulo { radio: 1.0 })];
    caso("con dyn Area", figuras.iter().map(|f| f.area()).sum(), 4.0);
    if ok { println!("TODOS LOS CASOS SUPERADOS"); }
}
