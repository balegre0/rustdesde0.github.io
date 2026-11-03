use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordenada {
    x: i32,
    y: i32,
}

fn huella<T: Hash>(t: &T) -> u64 {
    let mut h = DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

fn main() {
    // con Hash + Eq, el tipo puede ser clave de un HashMap
    let mut mapa = HashMap::new();
    mapa.insert(Coordenada { x: 0, y: 0 }, "origen");
    mapa.insert(Coordenada { x: 1, y: 2 }, "tesoro");
    println!("{:?}", mapa.get(&Coordenada { x: 1, y: 2 }));
    // valores iguales dan el mismo hash
    let a = huella(&Coordenada { x: 1, y: 2 });
    let b = huella(&Coordenada { x: 1, y: 2 });
    let c = huella(&Coordenada { x: 2, y: 1 });
    println!("{} {}", a == b, a == c);
}
