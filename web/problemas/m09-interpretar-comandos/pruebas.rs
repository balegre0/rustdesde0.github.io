
fn main() {
    let casos = [
        ("avanza 3", Some(Comando::Avanzar(3))),
        ("  avanza   12 ", Some(Comando::Avanzar(12))),
        ("gira izquierda", Some(Comando::Girar(Lado::Izquierda))),
        ("gira derecha", Some(Comando::Girar(Lado::Derecha))),
        ("para", Some(Comando::Parar)),
        ("avanza", None),
        ("avanza tres", None),
        ("avanza -2", None),
        ("gira arriba", None),
        ("", None),
        ("para ya", None),
    ];
    let mut fallos = 0;
    for (texto, esperado) in casos {
        let obtenido = parsear(texto);
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
