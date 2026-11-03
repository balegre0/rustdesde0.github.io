fn elegir<'a>(a: &'a str, b: &'a str, primero: bool) -> &'a str {
    if primero { a } else { b }
}

fn main() {
    println!("{}", elegir("x", "y", true));
}
