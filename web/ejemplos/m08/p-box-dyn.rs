use std::fmt::Display;

fn en_caja<T: Display>(t: T) -> Box<dyn Display> {
    Box::new(t)
}

fn main() {
    println!("{}", en_caja(5));
}
