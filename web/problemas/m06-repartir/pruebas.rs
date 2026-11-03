
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
    let casos: [(&[&str], &[&str], &[&str]); 3] = [
        (&["a", "b", "c", "d", "e"], &["a", "c", "e"], &["b", "d"]),
        (&["solo"], &["solo"], &[]),
        (&[], &[], &[]),
    ];
    for (entrada, ep, ei) in casos {
        let v: Vec<String> = entrada.iter().map(|s| s.to_string()).collect();
        // Guardamos dónde está cada texto en el heap para comprobar que no se clona.
        let direcciones: Vec<*const u8> = v.iter().map(|s| s.as_ptr()).collect();
        let (p, i) = repartir(v);
        let mismos = p.iter().chain(i.iter()).all(|s| s.is_empty() || direcciones.contains(&s.as_ptr()));
        let ok = p.iter().map(|s| s.as_str()).eq(ep.iter().copied()) && i.iter().map(|s| s.as_str()).eq(ei.iter().copied());
        comprobar(&format!("repartir({entrada:?})"), ok, format!("esperaba ({ep:?}, {ei:?}), obtuve ({p:?}, {i:?})"));
        comprobar(&format!("sin clonar en {entrada:?}"), mismos, String::from("algún texto está en otra dirección del heap: ¿has usado clone()?"));
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
