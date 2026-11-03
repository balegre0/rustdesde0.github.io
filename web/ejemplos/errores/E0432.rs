use std::colecciones::HashMap;

fn main() {
    let mapa: HashMap<&str, i32> = HashMap::new();
    println!("{}", mapa.len());
}
