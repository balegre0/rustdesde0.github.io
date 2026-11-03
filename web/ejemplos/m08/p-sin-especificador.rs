fn mas_larga(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    println!("{}", mas_larga("hola", "adiós"));
}
