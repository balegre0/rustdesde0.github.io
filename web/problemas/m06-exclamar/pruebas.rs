
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
    let casos = [("hola", 2, "hola!!"), ("año", 1, "año!"), ("", 3, "!!!"), ("ya", 0, "ya")];
    for (entrada, n, esperado) in casos {
        let obtenido = exclamar(String::from(entrada), n);
        comprobar(&format!("exclamar({entrada:?}, {n})"), obtenido == esperado, format!("esperaba {esperado:?}, obtuve {obtenido:?}"));
    }
    // La propiedad vuelve: podemos seguir usando el resultado.
    let s = exclamar(String::from("a"), 1);
    let s = exclamar(s, 1);
    comprobar("encadenar dos llamadas", s == "a!!", format!("obtuve {s:?}"));
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
