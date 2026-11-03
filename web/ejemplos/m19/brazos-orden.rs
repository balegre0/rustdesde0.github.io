macro_rules! que_es {
    (0) => { "el literal cero" };
    ($x:literal) => { "otro literal" };
    ($x:expr) => { "una expresión" };
}

fn main() {
    println!("{}", que_es!(0));
    println!("{}", que_es!(7));
    println!("{}", que_es!(3 + 4));
}
