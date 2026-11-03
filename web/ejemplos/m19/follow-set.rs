macro_rules! junta {
    ($a:expr $b:expr) => { $a + $b };
}

fn main() {
    println!("{}", junta!(1 2));
}
