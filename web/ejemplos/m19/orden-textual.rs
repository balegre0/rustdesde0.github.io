fn main() {
    println!("{}", triple!(4));
}

macro_rules! triple {
    ($e:expr) => { $e * 3 };
}
