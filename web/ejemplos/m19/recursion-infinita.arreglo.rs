macro_rules! cuenta_tokens {
    () => { 0 };
    ($x:tt $($resto:tt)*) => { 1 + cuenta_tokens!($($resto)*) };
}

fn main() {
    let n = cuenta_tokens!(a b c);
    println!("{}", n);
}
