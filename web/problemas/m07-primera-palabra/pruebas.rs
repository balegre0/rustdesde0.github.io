
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
    let casos = [("hola mundo", "hola"), ("España", "España"), (" empieza con espacio", ""), ("", ""), ("año nuevo", "año")];
    for (entrada, esperado) in casos {
        let obtenido = primera_palabra(entrada);
        comprobar(&format!("primera_palabra({entrada:?})"), obtenido == esperado, format!("esperaba {esperado:?}, obtuve {obtenido:?}"));
    }
    let frase = String::from("vista prestada");
    let p = primera_palabra(&frase);
    comprobar("devuelve un trozo del original (sin copiar)", p.as_ptr() == frase.as_ptr(), String::from("la vista no apunta al texto original"));
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
