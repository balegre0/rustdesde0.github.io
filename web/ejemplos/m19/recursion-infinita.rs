macro_rules! eterna {
    ($x:expr) => { eterna!($x + 1) };
}

fn main() {
    let n = eterna!(0);
    println!("{}", n);
}
