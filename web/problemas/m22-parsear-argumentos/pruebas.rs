fn main() {
    let op = |v: bool, m: Option<u32>, o: &str, d: &str| Ok(Opciones { verboso: v, max: m, origen: o.into(), destino: d.into() });
    let uso = String::from("uso: copiar [-v] [--max N] ORIGEN DESTINO");
    let casos: [(&[&str], Result<Opciones, String>); 7] = [
        (&["a.txt", "b.txt"], op(false, None, "a.txt", "b.txt")),
        (&["-v", "a", "b"], op(true, None, "a", "b")),
        (&["a", "--max", "10", "b", "-v"], op(true, Some(10), "a", "b")),
        (&["--max", "diez", "a", "b"], Err(String::from("--max necesita un número"))),
        (&["a", "b", "--max"], Err(String::from("--max necesita un número"))),
        (&["-x", "a", "b"], Err(String::from("opción desconocida: -x"))),
        (&["solo-uno"], Err(uso)),
    ];
    let mut fallos = 0;
    for (args, esperado) in casos {
        let obtenido = parsear(args);
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", args, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", args, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
