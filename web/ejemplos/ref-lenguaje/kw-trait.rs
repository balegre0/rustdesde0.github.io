// 1. Un trait describe un comportamiento: lo que un tipo sabe hacer.
trait Animal {
    fn nombre(&self) -> String;
    // 2. Método con implementación por defecto.
    fn presentarse(&self) -> String {
        format!("Soy {}", self.nombre())
    }
}

// 3. Supertrait: para ser Mascota hay que ser Animal.
trait Mascota: Animal {
    const PATAS: u32;
    fn dueno(&self) -> &str;
}

struct Perro { dueno: String }

impl Animal for Perro {
    fn nombre(&self) -> String { "un perro".into() }
}
impl Mascota for Perro {
    const PATAS: u32 = 4;
    fn dueno(&self) -> &str { &self.dueno }
}

// 4. Usar el trait como límite de un genérico.
fn describir<T: Mascota>(m: &T) -> String {
    format!("{} de {} con {} patas", m.presentarse(), m.dueno(), T::PATAS)
}

fn main() {
    let p = Perro { dueno: "Iris".into() };
    println!("{}", describir(&p));
}
