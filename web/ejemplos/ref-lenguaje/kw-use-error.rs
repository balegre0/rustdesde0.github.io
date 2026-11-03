fn main() {
    // Sin `use std::collections::HashMap;` el nombre no está en ámbito.
    let m: HashMap<u8, u8> = HashMap::new();
    println!("{}", m.len());
}
