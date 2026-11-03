use std::collections::HashMap;

fn obtener(mapa: &mut HashMap<u32, String>, k: u32) -> &String {
    match mapa.get(&k) {
        Some(v) => v,
        None => {
            mapa.insert(k, String::from("nuevo"));
            &mapa[&k]
        }
    }
}

fn main() {
    let mut m = HashMap::new();
    println!("{}", obtener(&mut m, 1));
}
