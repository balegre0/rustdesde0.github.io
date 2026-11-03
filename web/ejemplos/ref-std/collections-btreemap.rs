use std::collections::BTreeMap;

fn main() {
    let mut notas = BTreeMap::new();
    notas.insert("Luis", 7);
    notas.insert("Ana", 9);
    notas.insert("Eva", 5);
    // siempre en orden de clave
    for (nombre, nota) in &notas {
        println!("{nombre}: {nota}");
    }
    println!("{:?}", notas.first_key_value());
    println!("{:?}", notas.last_key_value());
    // rangos de claves
    let desde_b: Vec<_> = notas.range("B"..).collect();
    println!("{desde_b:?}");
    *notas.entry("Eva").or_insert(0) += 1;
    println!("{notas:?}");
}
