use std::fmt;

#[derive(Debug)]
struct Punto {
    x: i32,
    y: i32,
}

struct Secreto(String);

impl fmt::Debug for Secreto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // no enseña el contenido, solo cuántas letras tiene
        write!(f, "Secreto(*** {} letras)", self.0.len())
    }
}

#[derive(Debug)]
enum Forma {
    Circulo { radio: f64 },
    Vacia,
}

fn main() {
    let p = Punto { x: 1, y: -2 };
    println!("{p:?}");
    println!("x + y = {}", p.x + p.y);
    println!("{p:#?}");
    println!("{:?}", Secreto(String::from("clave1234")));
    let c = Forma::Circulo { radio: 1.5 };
    println!("{:?} {:?}", c, Forma::Vacia);
    if let Forma::Circulo { radio } = c {
        println!("radio: {radio}");
    }
    println!("{:?}", ("texto", 'c', [1.0, 2.5], Some(()), "con \"comillas\""));
}
