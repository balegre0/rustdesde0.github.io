use std::fmt::Debug;

// 1. Supertrait: para ser Mascota hay que ser también Debug.
trait Ser: Debug {
    fn nombre(&self) -> String;
}

trait Mascota: Ser {
    fn dueno(&self) -> String;
    // 2. Dentro de Mascota se pueden usar los métodos de Ser y de Debug.
    fn ficha(&self) -> String {
        format!("{} (de {}) {:?}", self.nombre(), self.dueno(), self)
    }
}

#[derive(Debug)]
struct Loro {
    palabras: u32,
}

impl Ser for Loro {
    fn nombre(&self) -> String {
        format!("Loro ({} palabras)", self.palabras)
    }
}

impl Mascota for Loro {
    fn dueno(&self) -> String {
        String::from("Lía")
    }
}

fn saluda(s: &dyn Ser) {
    println!("Hola, {}", s.nombre());
}

fn main() {
    let m: &dyn Mascota = &Loro { palabras: 12 };
    println!("{}", m.ficha());

    // 3. Upcasting (estable desde Rust 1.86): &dyn Mascota → &dyn Ser.
    let s: &dyn Ser = m;
    saluda(s);
}
