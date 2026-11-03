fn main() {
    let casos: [(&str, bool); 10] = [
        ("12345678Z", true),
        ("00000000T", true),
        ("99999999R", true),
        ("12345678z", false),
        ("12345678A", false),
        ("1234567Z", false),
        ("123456789Z", false),
        ("1234567AZ", false),
        ("", false),
        ("ñ2345678Z", false),
    ];
    let mut fallos = 0;
    for (dni, esperado) in casos {
        let obtenido = validar(dni);
        if obtenido == esperado {
            println!("OK: {dni:?} -> {obtenido}");
        } else {
            println!("FALLO: {dni:?}: esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    for (n, l) in [(0, 'T'), (22, 'E'), (23, 'T'), (12345678, 'Z')] {
        if letra(n) == l {
            println!("OK: letra({n}) = {l}");
        } else {
            println!("FALLO: letra({n}): esperaba {l}, obtuve {}", letra(n));
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
