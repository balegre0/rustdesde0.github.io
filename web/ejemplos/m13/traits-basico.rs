// 1. Un trait es una lista de capacidades: aquí, «tener área» y «tener nombre».
trait Forma {
    fn area(&self) -> f64;
    fn nombre(&self) -> String;

    // 2. Un método por defecto: lo heredan todos los tipos que no lo reescriban.
    fn describir(&self) -> String {
        format!("{} de área {:.2}", self.nombre(), self.area())
    }
}

struct Circulo {
    radio: f64,
}

struct Rectangulo {
    ancho: f64,
    alto: f64,
}

// 3. Cada tipo implementa el trait a su manera.
impl Forma for Circulo {
    fn area(&self) -> f64 {
        3.14159 * self.radio * self.radio
    }
    fn nombre(&self) -> String {
        String::from("círculo")
    }
}

impl Forma for Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
    fn nombre(&self) -> String {
        String::from("rectángulo")
    }
}

fn main() {
    let c = Circulo { radio: 1.0 };
    let r = Rectangulo { ancho: 2.0, alto: 3.5 };

    // 4. Los métodos del trait se llaman con punto, como cualquier método.
    println!("{}", c.describir());
    println!("{}", r.describir());

    // 5. También se pueden llamar con la ruta completa: Trait::metodo(&valor).
    println!("{}", Forma::area(&r));
}
