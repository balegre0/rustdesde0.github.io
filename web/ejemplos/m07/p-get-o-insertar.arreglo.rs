use std::collections::HashMap;

fn obtener(mapa: &mut HashMap<u32, String>, k: u32) -> &String {
    mapa.entry(k).or_insert_with(|| String::from("nuevo"))
}

fn main() {
    let mut m = HashMap::new();
    println!("{}", obtener(&mut m, 1));
}
