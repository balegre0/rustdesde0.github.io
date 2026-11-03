fn main() {
    let casos = [
        ("1.4.0", "1.9.2", true),
        ("1.4.0", "1.4.0", true),
        ("1.4.0", "1.3.9", false),
        ("1.4.0", "2.0.0", false),
        ("0.9.1", "0.9.7", true),
        ("0.9.1", "0.10.0", false),
        ("0.9.1", "0.9.0", false),
        ("0.0.3", "0.0.3", true),
        ("0.0.3", "0.0.4", false),
        ("2.0.0", "2.13.1", true),
    ];
    let mut fallos = 0;
    for (r, v, esperado) in casos {
        let obtenido = compatible(r, v);
        if obtenido == esperado {
            println!("OK: ^{r} con {v} -> {obtenido}");
        } else {
            println!("FALLO: ^{r} con {v}: esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
