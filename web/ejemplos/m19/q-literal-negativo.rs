macro_rules! clase {
    ($n:literal) => { "literal" };
    ($e:expr) => { "expresión" };
}

fn main() {
    println!("{}", clase!(5));
    println!("{}", clase!(-5));
    println!("{}", clase!(- 5));
    println!("{}", clase!(2 - 5));
}
