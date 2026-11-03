use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let mut visitados = HashSet::new();
    visitados.insert(Punto { x: 0, y: 0 });
    visitados.insert(Punto { x: 0, y: 0 });
    visitados.insert(Punto { x: 1, y: 2 });
    println!("{} puntos distintos", visitados.len());
}
