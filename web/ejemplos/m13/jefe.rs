use std::fmt;
use std::ops::Add;

// --- Un número con unidad: newtype + derive + operadores + Display ---
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct Area(f64);

impl Add for Area {
    type Output = Area;
    fn add(self, o: Area) -> Area {
        Area(self.0 + o.0)
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} m²", self.0)
    }
}

// --- El trait, con un método por defecto ---
trait Figura {
    fn nombre(&self) -> String;
    fn area(&self) -> Area;
    fn ficha(&self) -> String {
        format!("{:<10} {}", self.nombre(), self.area())
    }
}

struct Cuadrado {
    lado: f64,
}
struct Circulo {
    radio: f64,
}

impl Figura for Cuadrado {
    fn nombre(&self) -> String {
        String::from("cuadrado")
    }
    fn area(&self) -> Area {
        Area(self.lado * self.lado)
    }
}

impl Figura for Circulo {
    fn nombre(&self) -> String {
        String::from("círculo")
    }
    fn area(&self) -> Area {
        Area(3.14159 * self.radio * self.radio)
    }
}

// --- Conversión: de un número a un Cuadrado de ese lado ---
impl From<f64> for Cuadrado {
    fn from(lado: f64) -> Self {
        Cuadrado { lado }
    }
}

// --- Genérico: suma cualquier cosa que se pueda sumar y tenga un cero ---
fn total<T: Add<Output = T> + Default + Copy>(valores: &[T]) -> T {
    let mut s = T::default();
    for &v in valores {
        s = s + v;
    }
    s
}

fn main() {
    // dyn: figuras de tipos distintos en una sola lista.
    let figuras: Vec<Box<dyn Figura>> = vec![
        Box::new(Cuadrado::from(2.0)),
        Box::new(Circulo { radio: 1.0 }),
        Box::new(Cuadrado { lado: 0.5 }),
    ];

    for f in &figuras {
        println!("{}", f.ficha());
    }

    let areas: Vec<Area> = figuras.iter().map(|f| f.area()).collect();
    println!("Total: {}", total(&areas));

    // La misma función genérica sirve para enteros.
    println!("Total de lados: {}", total(&[4, 0, 4]));

    // PartialOrd derivado: compara las áreas.
    let mayor = areas.iter().copied().fold(Area::default(), |a, b| if b > a { b } else { a });
    println!("Mayor: {mayor}");
}
