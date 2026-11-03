// Completa la macro: max!(a) devuelve a; max!(a, b) devuelve el mayor.
macro_rules! max {
    ($a:expr) => { $a };
    // añade aquí el brazo para dos argumentos
}

fn main() {
    println!("{}", max!(7));
    // println!("{}", max!(3, 9));
    // println!("{}", max!(10 - 1, 2 * 2));
}
