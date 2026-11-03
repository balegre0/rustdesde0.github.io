// intercambia!(a, b) debe intercambiar los valores de dos variables.
// Usa una variable temporal DENTRO de la macro. Gracias a la higiene,
// no chocará con la `tmp` del usuario.
macro_rules! intercambia {
    ($a:ident, $b:ident) => {};
}

fn main() {
    let mut tmp = 1;
    let mut otro = 2;
    intercambia!(tmp, otro);
    println!("tmp = {}, otro = {}", tmp, otro);
}
