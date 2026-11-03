use std::collections::{BTreeMap, HashSet};

fn main() {
    let mut notas = BTreeMap::new();
    notas.insert("Zoe", 8);
    notas.insert("Ana", 9);
    notas.insert("Luis", 6);
    println!("{:?}", notas);

    let repetidos = [3, 1, 3, 2, 1, 3];
    let unicos: HashSet<i32> = repetidos.iter().copied().collect();
    println!("{}", unicos.len());
}
