// Drop check: si un tipo con referencias implementa Drop, lo prestado debe vivir más.
struct Inspector<'a>(&'a u8);

impl Drop for Inspector<'_> {
    fn drop(&mut self) {
        println!("inspeccionando {}", self.0); // ¡usa la referencia al destruirse!
    }
}

struct Mundo<'a> {
    inspector: Option<Inspector<'a>>,
    dias: Box<u8>,
}

fn main() {
    let mut mundo = Mundo { inspector: None, dias: Box::new(1) };
    mundo.inspector = Some(Inspector(&mundo.dias));
}
