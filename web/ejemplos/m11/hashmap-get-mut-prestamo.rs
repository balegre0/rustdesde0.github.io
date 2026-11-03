use std::collections::HashMap;

fn main() {
    let mut puntos: HashMap<&str, u32> = HashMap::new();
    puntos.insert("ana", 3);
    let p = puntos.get("ana").unwrap();
    puntos.insert("luis", 5);
    println!("{p}");
}
