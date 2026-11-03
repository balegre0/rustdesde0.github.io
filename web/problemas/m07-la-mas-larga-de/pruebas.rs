
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
    let casos: [(&[&str], Option<&str>); 4] = [
        (&["uno", "tres", "dos"], Some("tres")),
        (&["añil", "azul"], Some("añil")),
        (&[], None),
        (&["a", "bb", "cc"], Some("bb")),
    ];
    for (entrada, esperado) in casos {
        let v: Vec<String> = entrada.iter().map(|s| s.to_string()).collect();
        let obtenido = mas_larga_de(&v);
        comprobar(&format!("mas_larga_de({entrada:?})"), obtenido == esperado, format!("esperaba {esperado:?}, obtuve {obtenido:?}"));
        if let Some(r) = obtenido {
            comprobar("es una vista del vector", v.iter().any(|s| s.as_ptr() == r.as_ptr()), String::from("la vista no apunta a un texto del vector"));
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
