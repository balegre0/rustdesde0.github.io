
fn main() {
    let mut fallos = 0;
    let mut caso = |nombre: &str, ok: bool| {
        if ok { println!("OK: {nombre}"); } else { println!("FALLO: {nombre}"); fallos += 1; }
    };
    let t = TextoAscii::nuevo("Hola");
    caso("nuevo ASCII", t.as_ref().map(|t| t.como_str()) == Some("Hola"));
    caso("nuevo con ñ", TextoAscii::nuevo("año").is_none());
    caso("nuevo vacío", TextoAscii::nuevo("").map(|t| t.como_str().len()) == Some(0));
    let mut t = t.unwrap();
    caso("empujar '!'", t.empujar('!'));
    caso("empujar 'ñ' falla", !t.empujar('ñ'));
    caso("empujar '🦀' falla", !t.empujar('🦀'));
    caso("contenido final", t.como_str() == "Hola!");
    if fallos == 0 { println!("TODOS LOS CASOS SUPERADOS"); }
}
