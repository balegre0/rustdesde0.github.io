use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    for (i, letra) in ["a", "b", "c", "d", "e", "f"].iter().enumerate() {
        m.insert(*letra, i);
    }
    let orden: Vec<_> = m.keys().collect();
    println!("{:?}", orden);
}
