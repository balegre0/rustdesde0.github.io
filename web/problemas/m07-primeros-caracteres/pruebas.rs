
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
    let casos = [("año nuevo", 3, "año"), ("ñu", 1, "ñ"), ("hola", 10, "hola"), ("hola", 0, ""), ("", 2, ""), ("¿qué?", 2, "¿q")];
    for (s, n, esperado) in casos {
        let obtenido = primeros(s, n);
        comprobar(&format!("primeros({s:?}, {n})"), obtenido == esperado, format!("esperaba {esperado:?}, obtuve {obtenido:?}"));
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
