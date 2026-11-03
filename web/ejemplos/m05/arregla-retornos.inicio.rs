// Este programa no compila: hay dos errores de «sentencia en vez de expresión».
// Arréglalos sin añadir `return`.
fn media(a: f64, b: f64) -> f64 {
    (a + b) / 2.0;
}

fn aprobado(nota: f64) -> bool {
    let resultado = nota >= 5.0;
    resultado;
}

fn main() {
    let m = media(4.0, 7.0);
    println!("media = {m}, ¿aprobado? {}", aprobado(m));
}
