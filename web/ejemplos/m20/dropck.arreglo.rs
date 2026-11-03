// Sin Drop en Inspector, el compilador sabe que nadie usará la referencia al destruir.
struct Inspector<'a>(&'a u8);

struct Mundo<'a> {
    inspector: Option<Inspector<'a>>,
    dias: Box<u8>,
}

fn main() {
    let mut mundo = Mundo { inspector: None, dias: Box::new(1) };
    mundo.inspector = Some(Inspector(&mundo.dias));
    if let Some(i) = &mundo.inspector {
        println!("días = {}", i.0);
    }
}
