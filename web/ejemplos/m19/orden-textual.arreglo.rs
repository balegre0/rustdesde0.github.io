macro_rules! triple {
    ($e:expr) => { $e * 3 };
}

fn main() {
    println!("{}", triple!(4));
}
