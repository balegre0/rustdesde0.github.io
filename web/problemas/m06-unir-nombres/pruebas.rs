
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
    let casos: [(&[&str], &str); 4] = [
        (&["Ana", "Luis", "Eva"], "Ana, Luis, Eva"),
        (&["Íñigo"], "Íñigo"),
        (&[], ""),
        (&["", "x"], ", x"),
    ];
    for (nombres, esperado) in casos {
        let v: Vec<String> = nombres.iter().map(|s| s.to_string()).collect();
        let obtenido = unir(v);
        comprobar(&format!("unir({nombres:?})"), obtenido == esperado, format!("esperaba {esperado:?}, obtuve {obtenido:?}"));
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
