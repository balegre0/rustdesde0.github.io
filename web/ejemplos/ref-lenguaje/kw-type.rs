use std::collections::HashMap;

// 1. Alias de tipo: otro nombre para un tipo que ya existe.
type Agenda = HashMap<String, Vec<u32>>;
type Resultado<T> = Result<T, String>;

fn dividir(a: i32, b: i32) -> Resultado<i32> {
    if b == 0 { Err("división entre cero".into()) } else { Ok(a / b) }
}

// 2. Tipo asociado en un trait: el implementador elige el tipo.
trait Contenedor {
    type Elem;
    fn primero(&self) -> Option<&Self::Elem>;
}

struct Caja(Vec<char>);

impl Contenedor for Caja {
    type Elem = char;
    fn primero(&self) -> Option<&char> { self.0.first() }
}

fn main() {
    let mut a: Agenda = HashMap::new();
    a.insert("Ana".into(), vec![600, 700]);
    println!("{:?}", a.get("Ana"));
    println!("{:?} {:?}", dividir(7, 2), dividir(1, 0));
    println!("{:?}", Caja(vec!['x', 'y']).primero());
}
