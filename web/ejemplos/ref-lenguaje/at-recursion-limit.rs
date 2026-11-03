#![recursion_limit = "256"]

// Una macro que se llama a sí misma una vez por cada token: con muchos tokens,
// supera el límite por defecto (128) y hay que subirlo.
macro_rules! contar {
    () => { 0usize };
    ($cabeza:tt $($cola:tt)*) => { 1usize + contar!($($cola)*) };
}

fn main() {
    let n = contar!(a b c d e f g h i j);
    println!("{n} tokens");
}
