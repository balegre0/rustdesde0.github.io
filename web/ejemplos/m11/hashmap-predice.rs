use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock.insert("manzanas", 10);
    stock.insert("peras", 4);
    stock.insert("manzanas", 7);
    println!("{:?} {:?} {}", stock.get("manzanas"), stock.get("kiwis"), stock.len());
}
