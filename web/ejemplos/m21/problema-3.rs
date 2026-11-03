use std::collections::HashMap;

// Devuelve el valor de `k`; si no existe, lo crea primero.
fn obtener_o_crear(mapa: &mut HashMap<u32, String>, k: u32) -> &String {
    if let Some(v) = mapa.get(&k) {
        return v;
    }
    mapa.insert(k, String::from("nuevo"));
    mapa.get(&k).unwrap()
}

fn main() {
    let mut m = HashMap::new();
    m.insert(1, String::from("uno"));
    println!("{}", obtener_o_crear(&mut m, 1));
    println!("{}", obtener_o_crear(&mut m, 2));
}
