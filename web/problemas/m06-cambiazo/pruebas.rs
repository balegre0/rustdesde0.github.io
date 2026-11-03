
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
    let casos: [(&[&str], usize, &[&str], &[&str]); 4] = [
        (&["hola", "sí", "camión"], 3, &["hola", "camión"], &["", "sí", ""]),
        (&["año", "años"], 3, &["años"], &["año", ""]),
        (&["a"], 5, &[], &["a"]),
        (&[], 0, &[], &[]),
    ];
    for (antes, max, esperado, despues) in casos {
        let mut v: Vec<String> = antes.iter().map(|s| s.to_string()).collect();
        let direcciones: Vec<*const u8> = v.iter().map(|s| s.as_ptr()).collect();
        let sacados = retirar_largos(&mut v, max);
        let ok = sacados.iter().map(|s| s.as_str()).eq(esperado.iter().copied())
            && v.iter().map(|s| s.as_str()).eq(despues.iter().copied());
        comprobar(&format!("retirar_largos({antes:?}, {max})"), ok, format!("esperaba {esperado:?} y {despues:?}, obtuve {sacados:?} y {v:?}"));
        let sin_clonar = sacados.iter().all(|s| direcciones.contains(&s.as_ptr()));
        comprobar(&format!("sin clonar en {antes:?}"), sin_clonar, String::from("algún texto sacado está en otra dirección: ¿has clonado?"));
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
