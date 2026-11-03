fn elegir(a: &str, b: &str, primero: bool) -> &str {
    if primero { a } else { b }
}

fn main() {
    println!("{}", elegir("x", "y", true));
}
