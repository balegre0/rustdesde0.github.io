use std::collections::BTreeMap;

fn main() {
    let partidas = [("Ana", 120), ("Luis", 300), ("Zoe", 250)];
    let mut ranking: BTreeMap<u32, &str> = BTreeMap::new();
    for (nombre, puntos) in partidas {
        ranking.insert(puntos, nombre);
    }
    for (puntos, nombre) in ranking.iter().rev() {
        println!("{puntos} {nombre}");
    }
}
