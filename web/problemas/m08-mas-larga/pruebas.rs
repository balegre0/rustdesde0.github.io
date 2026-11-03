
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
    let casos = [("sol", "luna", "luna"), ("mar", "río", "río"), ("ab", "cd", "ab"), ("", "", "")];
    for (a, b, esperado) in casos {
        let obtenido = mas_larga(a, b);
        comprobar(&format!("mas_larga({a:?}, {b:?})"), obtenido == esperado, format!("esperaba {esperado:?}, obtuve {obtenido:?}"));
    }
    let largo = String::from("larguísimo");
    {
        let corto = String::from("x");
        let r = mas_larga(&largo, &corto);
        comprobar("con dos String prestados", r == "larguísimo", format!("obtuve {r:?}"));
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
