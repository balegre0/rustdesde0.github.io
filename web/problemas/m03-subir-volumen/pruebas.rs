
fn main() {
    let casos: [(u8, u8, u8); 8] = [
        (0, 3, 30),
        (240, 1, 250),
        (250, 1, 255),
        (10, 200, 255),
        (0, 0, 0),
        (255, 0, 255),
        (0, 25, 250),
        (0, 26, 255),
    ];
    let mut fallos = 0;
    for (volumen, pulsaciones, esperado) in casos {
        let obtenido = subir_volumen(volumen, pulsaciones);
        if obtenido == esperado {
            println!("OK: subir_volumen({volumen}, {pulsaciones}) = {obtenido}");
        } else {
            println!("FALLO: subir_volumen({volumen}, {pulsaciones}): esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
