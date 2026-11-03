use std::fmt::Display;

fn guardar<T: Display>(t: T) -> Box<dyn Display> {
    Box::new(t)
}

fn main() {
    println!("{}", guardar(5));
}
