use std::collections::HashMap;

fn main() {
    let texto = "año nuevo";
    let mut cuenta: HashMap<char, usize> = HashMap::new();
    for c in texto.chars() {
        if c != ' ' {
            *cuenta.entry(c).or_insert(0) += 1;
        }
    }

    let mut pares: Vec<_> = cuenta.into_iter().collect();
    pares.sort();
    println!("{:?}", pares);
}
