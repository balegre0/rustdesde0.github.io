fn main() {
    let casos = [
        ("hola mundo", "aloh odnum"),
        ("año nuevo", "oña oveun"),
        ("a", "a"),
        ("", ""),
        ("Rust es genial", "tsuR se laineg"),
    ];
    let mut fallos = 0;
    for (entrada, esperado) in casos {
        let obtenido = invertir_palabras(entrada);
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", entrada, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", entrada, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
