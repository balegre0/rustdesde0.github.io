fn gritar(s: &str) -> &str {
    let grande = s.to_uppercase();
    &grande
}

fn main() {
    println!("{}", gritar("hola"));
}
