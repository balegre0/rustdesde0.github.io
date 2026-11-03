
fn main() {
    let casos: [(&str, (usize, usize)); 7] = [
        ("hola", (4, 4)),
        ("año", (4, 3)),
        ("€", (3, 1)),
        ("", (0, 0)),
        ("Begoña Ibáñez", (16, 13)),
        ("🦀🦀", (8, 2)),
        ("ñoño", (6, 4)),
    ];
    let mut fallos = 0;
    for (texto, esperado) in casos {
        let obtenido = medir(texto);
        if obtenido == esperado {
            println!("OK: medir({:?}) = {:?}", texto, obtenido);
        } else {
            println!("FALLO: medir({:?}): esperaba {:?}, obtuve {:?}", texto, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
