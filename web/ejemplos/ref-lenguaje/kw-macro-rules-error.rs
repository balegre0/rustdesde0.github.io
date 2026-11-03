macro_rules! doble {
    ($x:expr) => { $x * 2 };
}

fn main() {
    // Ninguna regla acepta dos argumentos.
    println!("{}", doble!(1, 2));
}
