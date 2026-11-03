use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Punto {
    x: i32,
    y: i32,
}

// Hash a mano: solo cuenta el nombre, sin mayúsculas.
struct Etiqueta(String);

impl Hash for Etiqueta {
    fn hash<H: Hasher>(&self, estado: &mut H) {
        self.0.to_lowercase().hash(estado);
    }
}

fn hash_de<T: Hash>(t: &T) -> u64 {
    let mut h = DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}

fn main() {
    println!("mismo hash para puntos iguales: {}", hash_de(&Punto { x: 1, y: 2 }) == hash_de(&Punto { x: 1, y: 2 }));
    println!("mismo hash con mayúsculas distintas: {}", hash_de(&Etiqueta("Rojo".into())) == hash_de(&Etiqueta("ROJO".into())));
    let mut visitas = HashMap::new();
    *visitas.entry(Punto { x: 0, y: 0 }).or_insert(0) += 1;
    *visitas.entry(Punto { x: 0, y: 0 }).or_insert(0) += 1;
    println!("{visitas:?}");
}
