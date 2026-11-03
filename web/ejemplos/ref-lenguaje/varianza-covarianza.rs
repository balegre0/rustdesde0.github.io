// `&'a str` es covariante en 'a: un &'static str sirve donde se pide un &'a str.
fn el_mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// `fn(T)` es contravariante: una función que acepta CUALQUIER &str
// sirve donde se pide una que acepte solo &'static str.
fn aplicar(f: fn(&'static str) -> usize) -> usize {
    f("estático")
}
fn contar(s: &str) -> usize { s.chars().count() }

fn main() {
    let local = String::from("una cadena local");
    let fijo: &'static str = "corta";
    // 'static se «acorta» al tiempo de vida de `local`.
    println!("{}", el_mas_largo(&local, fijo));
    println!("{}", aplicar(contar));
}
