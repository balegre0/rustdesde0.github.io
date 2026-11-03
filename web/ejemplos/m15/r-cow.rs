use std::borrow::Cow;

// Añade un punto final solo si falta.
fn con_punto(frase: &str) -> Cow<'_, str> {
    if frase.ends_with('.') {
        Cow::Borrowed(frase)
    } else {
        Cow::Owned(format!("{frase}."))
    }
}

fn main() {
    for f in ["Hola.", "Adiós"] {
        let r = con_punto(f);
        let tipo = if matches!(r, Cow::Borrowed(_)) { "prestado" } else { "propio" };
        println!("{r} ({tipo})");
    }
}
