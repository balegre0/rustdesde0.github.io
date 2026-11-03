// Completa las tres funciones usando SOLO iteradores y closures (sin bucles).
// Pista: texto.split_whitespace() da un iterador de &str.

// 1. Longitud (en caracteres, no en bytes) de cada palabra.
fn longitudes(texto: &str) -> Vec<usize> {
    todo!()
}

// 2. La palabra más larga (si hay empate, la última). Pista: max_by_key.
fn mas_larga(texto: &str) -> Option<&str> {
    todo!()
}

// 3. Cuántas palabras cumplen `condicion`.
fn contar_si(texto: &str, condicion: impl Fn(&str) -> bool) -> usize {
    todo!()
}

fn main() {
    let texto = "el pingüino come ñoquis con su pareja";
    println!("{:?}", longitudes(texto));
    println!("{:?}", mas_larga(texto));
    println!("{:?}", mas_larga(""));
    let minimo = 4;
    println!("{}", contar_si(texto, |p| p.chars().count() >= minimo));
    println!("{}", contar_si(texto, |p| p.starts_with('c')));
}
