
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
    let casos = [
        ("clave=valor", '=', ("clave", "valor")),
        ("a=b=c", '=', ("a", "b=c")),
        ("sin", '=', ("sin", "")),
        ("año:2026", ':', ("año", "2026")),
        ("xñy", 'ñ', ("x", "y")),
        ("=", '=', ("", "")),
    ];
    for (s, sep, esperado) in casos {
        let obtenido = dividir(s, sep);
        comprobar(&format!("dividir({s:?}, {sep:?})"), obtenido == esperado, format!("esperaba {esperado:?}, obtuve {obtenido:?}"));
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
