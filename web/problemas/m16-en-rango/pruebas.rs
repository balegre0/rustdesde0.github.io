
fn main() {
    let casos = [
        ((5, 1, 10), true, "en medio"),
        ((1, 1, 10), true, "justo en el mínimo"),
        ((10, 1, 10), true, "justo en el máximo"),
        ((0, 1, 10), false, "uno por debajo"),
        ((11, 1, 10), false, "uno por encima"),
        ((7, 7, 7), true, "rango de un solo número"),
        ((3, 5, 2), false, "rango vacío (min > max)"),
        ((-5, -10, -1), true, "negativos"),
        ((i32::MAX, 0, i32::MAX), true, "el mayor i32"),
        ((i32::MIN, i32::MIN + 1, 0), false, "el menor i32, fuera"),
    ];
    let mut fallos = 0;
    for ((x, min, max), esperado, nombre) in casos {
        let obtenido = en_rango(x, min, max);
        if obtenido == esperado {
            println!("OK: {nombre}: en_rango({x}, {min}, {max}) = {obtenido}");
        } else {
            println!("FALLO: {nombre}: en_rango({x}, {min}, {max}) debía ser {esperado}, salió {obtenido}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
