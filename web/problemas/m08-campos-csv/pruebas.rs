
fn main() {
    let mut fallos = 0;
    let mut comprobar = |nombre: &str, ok: bool, detalle: String| {
        if ok {
            println!("OK: {nombre}");
        } else {
            println!("FALLO: {nombre}: {detalle}");
            fallos += 1;
        }
    };
    let linea = String::from("año,mes,,día");
    let mut c = Campos { resto: Some(&linea) };
    // Guardamos varios campos a la vez mientras seguimos pidiendo: requiere Option<&'a str>.
    let a = c.siguiente();
    let b = c.siguiente();
    let d = c.siguiente();
    let e = c.siguiente();
    let f = c.siguiente();
    let g = c.siguiente();
    let todos = [a, b, d, e, f, g];
    let esperado = [Some("año"), Some("mes"), Some(""), Some("día"), None, None];
    comprobar("año,mes,,día", todos == esperado, format!("obtuve {todos:?}"));
    let mut c = Campos { resto: Some("") };
    let x = (c.siguiente(), c.siguiente());
    comprobar("línea vacía: un campo vacío", x == (Some(""), None), format!("obtuve {x:?}"));
    let mut c = Campos { resto: Some("x,") };
    let x = (c.siguiente(), c.siguiente(), c.siguiente());
    comprobar("coma final", x == (Some("x"), Some(""), None), format!("obtuve {x:?}"));
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
