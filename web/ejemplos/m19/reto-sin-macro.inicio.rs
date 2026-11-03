// Esta macro evalúa su argumento dos veces. Sustitúyela por una función
// `fn media(a: f64, b: f64) -> f64` y cambia las llamadas.
macro_rules! media {
    ($a:expr, $b:expr) => { ($a + $b) / 2.0 };
}

fn main() {
    println!("{}", media!(3.0, 4.0));
    println!("{}", media!(10.0, -2.0));
}
