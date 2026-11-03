
fn main() {
    let casos: [(u8, u8, Option<u8>); 7] = [
        (100, 50, Some(150)),
        (200, 55, Some(255)),
        (200, 56, None),
        (255, 255, None),
        (0, 0, Some(0)),
        (255, 0, Some(255)),
        (1, 255, None),
    ];
    let mut fallos = 0;
    for (a, b, esperado) in casos {
        let obtenido = suma_segura(a, b);
        if obtenido == esperado {
            println!("OK: suma_segura({a}, {b}) = {:?}", obtenido);
        } else {
            println!("FALLO: suma_segura({a}, {b}): esperaba {:?}, obtuve {:?}", esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
