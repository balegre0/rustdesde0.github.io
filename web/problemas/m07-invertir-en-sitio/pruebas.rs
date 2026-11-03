
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
    let casos: [(&[&str], &[&str]); 4] = [
        (&["a", "b", "c", "d"], &["d", "c", "b", "a"]),
        (&["x", "y", "z"], &["z", "y", "x"]),
        (&[], &[]),
        (&["solo"], &["solo"]),
    ];
    for (antes, despues) in casos {
        let mut v: Vec<String> = antes.iter().map(|s| s.to_string()).collect();
        let direcciones: Vec<*const u8> = v.iter().rev().map(|s| s.as_ptr()).collect();
        invertir(&mut v);
        let ok = v.iter().map(|s| s.as_str()).eq(despues.iter().copied());
        comprobar(&format!("invertir({antes:?})"), ok, format!("esperaba {despues:?}, obtuve {v:?}"));
        let mismos: Vec<*const u8> = v.iter().map(|s| s.as_ptr()).collect();
        comprobar(&format!("sin clonar en {antes:?}"), mismos == direcciones, String::from("los textos están en otras direcciones: ¿has clonado?"));
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
