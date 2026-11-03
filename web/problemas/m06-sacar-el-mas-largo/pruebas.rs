
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
    let casos: [(&[&str], Option<&str>, &[&str]); 4] = [
        (&["sol", "luna", "mar"], Some("luna"), &["sol", "mar"]),
        (&["ab", "cd"], Some("ab"), &["cd"]),
        (&[], None, &[]),
        (&["a", "año", "ojo"], Some("año"), &["a", "ojo"]),
    ];
    for (antes, esperado, despues) in casos {
        let mut v: Vec<String> = antes.iter().map(|s| s.to_string()).collect();
        let obtenido = sacar_mas_largo(&mut v);
        let ok = obtenido.as_deref() == esperado && v.iter().map(|s| s.as_str()).eq(despues.iter().copied());
        comprobar(&format!("sacar_mas_largo({antes:?})"), ok, format!("esperaba {esperado:?} y {despues:?}, obtuve {obtenido:?} y {v:?}"));
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
