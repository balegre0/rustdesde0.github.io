macro_rules! cuenta_tt {
    () => { 0 };
    ($t:tt $($resto:tt)*) => { 1 + cuenta_tt!($($resto)*) };
}

fn main() {
    println!("{}", cuenta_tt!(a + b));
    println!("{}", cuenta_tt!((a + b) [c d] {e}));
    println!("{}", cuenta_tt!(x ==> y));
}
