fn main() {
    let casos: [(&str, Option<(&str, usize)>); 6] = [
        ("el gato y el perro", Some(("el", 2))),
        ("Sol sol SOL luna", Some(("sol", 3))),
        ("b a", Some(("a", 1))),
        ("", None),
        ("   \n\t ", None),
        ("ñu Ñu año", Some(("ñu", 2))),
    ];
    let mut fallos = 0;
    for (texto, esperado) in casos {
        let obtenido = mas_frecuente(texto);
        let esperado = esperado.map(|(p, n)| (p.to_string(), n));
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", texto, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", texto, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
