use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let mut visitados = HashSet::new();
    visitados.insert(Punto { x: 0, y: 0 });
    println!("{:?}", visitados);
}
