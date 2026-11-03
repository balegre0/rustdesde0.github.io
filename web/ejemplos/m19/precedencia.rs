macro_rules! doble {
    ($x:expr) => { $x * 2 };
}

fn main() {
    let r = doble!(3 + 1);
    println!("{}", r);
}
