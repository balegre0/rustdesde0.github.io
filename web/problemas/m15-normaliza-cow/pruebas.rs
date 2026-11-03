
fn main() {
    // (entrada, salida esperada, ¿debe ser Borrowed?)
    let casos = [
        ("hola mundo", "hola mundo", true),
        ("hola   mundo", "hola mundo", false),
        ("  año  nuevo ", " año nuevo ", false),
        ("", "", true),
        ("sin_espacios", "sin_espacios", true),
        ("a  b  c    d", "a b c d", false),
        (" uno ", " uno ", true),
        ("ñandú     y   pingüino", "ñandú y pingüino", false),
    ];
    let mut fallos = 0;
    for (entrada, esperado, prestado) in casos {
        let r = normaliza(entrada);
        let es_prestado = matches!(r, std::borrow::Cow::Borrowed(_));
        let variante = if es_prestado { "Borrowed" } else { "Owned" };
        if r == esperado && es_prestado == prestado {
            println!("OK: {entrada:?} → {r:?} ({variante})");
        } else {
            let quiero = if prestado { "Borrowed" } else { "Owned" };
            println!("FALLO: {entrada:?} → {r:?} ({variante}); se esperaba {esperado:?} ({quiero})");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
