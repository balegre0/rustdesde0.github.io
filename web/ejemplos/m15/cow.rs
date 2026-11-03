use std::borrow::Cow;

// 1. Si no hay nada que cambiar, devolvemos un préstamo (sin reservar memoria).
//    Si hay que cambiar algo, devolvemos un String nuevo.
fn sin_espacios(texto: &str) -> Cow<'_, str> {
    if texto.contains(' ') {
        Cow::Owned(texto.replace(' ', "_"))
    } else {
        Cow::Borrowed(texto)
    }
}

fn describe(c: &Cow<str>) -> &'static str {
    match c {
        Cow::Borrowed(_) => "prestado",
        Cow::Owned(_) => "propio",
    }
}

fn main() {
    for t in ["hola", "hola mundo", "año_nuevo"] {
        let r = sin_espacios(t);
        // 2. Cow se usa como un &str gracias a Deref.
        println!("{t} -> {r} ({}, {} letras)", describe(&r), r.chars().count());
    }
}
