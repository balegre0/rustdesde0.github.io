fn main() {
    let mut fallos = 0;
    for n in [0u8, 1, 5, 42, 128, 200, 255] {
        let esperado = format!("{n:08b}");
        let obtenido = a_binario(n);
        if obtenido == esperado {
            println!("OK: {n} -> {obtenido}");
        } else {
            println!("FALLO: {n}: esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
