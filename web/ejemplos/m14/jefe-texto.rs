// 1. Longitud (en caracteres, no en bytes) de cada palabra.
fn longitudes(texto: &str) -> Vec<usize> {
    texto.split_whitespace().map(|p| p.chars().count()).collect()
}

// 2. La palabra más larga (si hay empate, max_by_key se queda con la última).
fn mas_larga(texto: &str) -> Option<&str> {
    texto.split_whitespace().max_by_key(|p| p.chars().count())
}

// 3. Cuántas palabras cumplen una condición que decide quien llama.
fn contar_si(texto: &str, condicion: impl Fn(&str) -> bool) -> usize {
    texto.split_whitespace().filter(|p| condicion(p)).count()
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
