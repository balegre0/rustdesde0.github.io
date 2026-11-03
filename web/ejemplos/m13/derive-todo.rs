use std::collections::HashSet;

// 1. `derive` le pide al compilador que escriba las impl por ti.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Carta {
    palo: char,
    valor: u8,
}

// 2. Copy solo se puede derivar si TODOS los campos son Copy.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Punto {
    x: f64,
    y: f64,
}

fn main() {
    let a = Carta { palo: '♥', valor: 7 };
    let b = a.clone(); // Clone: copia explícita
    println!("{a:?}"); // Debug con {:?}
    println!("{b:#?}"); // Debug «bonito» con {:#?}
    println!("a == b: {}", a == b); // PartialEq: campo a campo

    // Ord derivado compara campo a campo EN ORDEN: primero palo, luego valor.
    let c = Carta { palo: '♠', valor: 1 };
    println!("a < c: {}", a < c);

    // Hash + Eq permiten meter Cartas en un HashSet.
    let mut mano = HashSet::new();
    mano.insert(a.clone());
    mano.insert(b);
    println!("cartas distintas: {}", mano.len());

    // Default: cada campo con su valor por defecto ('\0' y 0).
    println!("{:?}", Carta::default());

    // Copy: asignar copia; `p` sigue siendo usable.
    let p = Punto { x: 1.0, y: 2.0 };
    let q = p;
    println!("{p:?} {q:?}");
}
