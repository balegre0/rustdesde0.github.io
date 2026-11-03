mod macros {
    macro_rules! area {
        ($b:expr, $h:expr) => { $b * $h };
    }
    pub(crate) use area;
}

mod figuras {
    use crate::macros::area;
    pub fn rectangulo() -> u32 { area!(3, 4) }
}

fn main() {
    println!("{}", figuras::rectangulo());
    println!("{}", crate::macros::area!(5, 2));
}
