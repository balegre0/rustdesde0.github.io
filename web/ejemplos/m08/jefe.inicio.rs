/// Recorre las palabras de un texto sin copiarlas: cada una es una slice del original.
/// Completa las anotaciones de lifetime que faltan (busca los `/* ? */`) y el cuerpo de `siguiente`.
struct Palabras<'a> {
    resto: &'a str,
}

impl<'a> Palabras<'a> {
    fn new(texto: &'a str) -> Self {
        Palabras { resto: texto }
    }

    /// Devuelve la siguiente palabra (separadas por espacios) o None si no quedan.
    /// Pista: trim_start(), find(' ') y dos slices del texto.
    fn siguiente(&mut self) -> Option<&/* ? */ str> {
        todo!()
    }
}

/// La palabra con más bytes del texto (la primera si hay empate).
fn mas_larga(texto: &str) -> &str {
    let mut p = Palabras::new(texto);
    let mut mejor = "";
    while let Some(w) = p.siguiente() {
        if w.len() > mejor.len() {
            mejor = w;
        }
    }
    mejor
}

fn main() {
    let texto = String::from("  el murciélago   come kiwi ");
    let mut p = Palabras::new(&texto);
    let a = p.siguiente();
    let b = p.siguiente();
    println!("{a:?} {b:?}");
    let todas: Vec<&str> = {
        let mut v = Vec::new();
        let mut q = Palabras::new(&texto);
        while let Some(w) = q.siguiente() {
            v.push(w);
        }
        v
    };
    println!("{todas:?}");
    println!("más larga: {}", mas_larga(&texto));
}
