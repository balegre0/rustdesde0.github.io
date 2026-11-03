use std::collections::HashMap;

// Arreglo para el borrow checker estable: preguntar primero sin devolver el préstamo.
fn obtener_o_crear(mapa: &mut HashMap<u32, String>, k: u32) -> &String {
    if !mapa.contains_key(&k) {
        mapa.insert(k, String::from("nuevo"));
    }
    mapa.get(&k).unwrap()
}

fn main() {
    let mut m = HashMap::new();
    m.insert(1, String::from("uno"));
    println!("{}", obtener_o_crear(&mut m, 1));
    println!("{}", obtener_o_crear(&mut m, 2));
}
