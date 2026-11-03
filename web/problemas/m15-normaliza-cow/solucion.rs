use std::borrow::Cow;

fn normaliza(texto: &str) -> Cow<'_, str> {
    if !texto.contains("  ") {
        return Cow::Borrowed(texto);
    }
    let mut salida = String::with_capacity(texto.len());
    let mut anterior_espacio = false;
    for c in texto.chars() {
        if c == ' ' && anterior_espacio {
            continue;
        }
        anterior_espacio = c == ' ';
        salida.push(c);
    }
    Cow::Owned(salida)
}
