// En la edición 2024, `expr` también acepta `_` y bloques `const { ... }`.
// `expr_2021` conserva el comportamiento antiguo.
macro_rules! nuevo {
    ($e:expr) => { "expr" };
    ($t:tt) => { "otra cosa" };
}
macro_rules! viejo {
    ($e:expr_2021) => { "expr_2021" };
    ($($t:tt)*) => { "otra cosa" };
}

fn main() {
    println!("nuevo!(_)            -> {}", nuevo!(_));
    println!("viejo!(_)            -> {}", viejo!(_));
    println!("nuevo!(const {{ 1 }}) -> {}", nuevo!(const { 1 }));
    println!("viejo!(const {{ 1 }}) -> {}", viejo!(const { 1 }));
}
