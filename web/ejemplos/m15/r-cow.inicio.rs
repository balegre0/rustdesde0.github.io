use std::borrow::Cow;

// Añade un punto final solo si falta.
// Si ya lo tiene, devuelve Cow::Borrowed; si no, Cow::Owned con format!.
fn con_punto(frase: &str) -> Cow<'_, str> {
    let _ = frase;
    todo!()
}

fn main() {
    for f in ["Hola.", "Adiós"] {
        let r = con_punto(f);
        let tipo = if matches!(r, Cow::Borrowed(_)) { "prestado" } else { "propio" };
        println!("{r} ({tipo})");
    }
}
