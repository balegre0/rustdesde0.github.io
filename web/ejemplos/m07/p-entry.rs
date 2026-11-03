use std::collections::HashMap;

fn main() {
    let mut cuenta: HashMap<char, u32> = HashMap::new();
    for c in "banana".chars() {
        *cuenta.entry(c).or_insert(0) += 1;
    }
    println!("a={} n={} b={}", cuenta[&'a'], cuenta[&'n'], cuenta[&'b']);
}
