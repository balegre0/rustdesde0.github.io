fn main() {
    let texto = "Rust es rápido.\nY seguro.\nOxidarse: lo que le pasa al hierro, no a RUST.\n";
    let casos = [
        ("Rust", false, vec![(1, "Rust es rápido.")]),
        ("rust", true, vec![(1, "Rust es rápido."), (3, "Oxidarse: lo que le pasa al hierro, no a RUST.")]),
        ("seguro", false, vec![(2, "Y seguro.")]),
        ("ñandú", true, vec![]),
    ];
    let mut fallos = 0;
    for (q, ignorar, esperado) in casos {
        let obtenido = buscar(q, Cursor::new(texto), ignorar);
        let esperado: Vec<(usize, String)> = esperado.into_iter().map(|(n, s)| (n, s.to_string())).collect();
        if obtenido == esperado {
            println!("OK: {q:?} (ignorar={ignorar}) -> {:?}", obtenido);
        } else {
            println!("FALLO: {q:?}: esperaba {:?}, obtuve {:?}", esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
