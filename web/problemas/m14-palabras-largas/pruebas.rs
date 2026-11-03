fn main() {
    let casos: [(&str, usize, &[&str]); 5] = [
        ("el ñandú corre mucho", 5, &["ÑANDÚ", "CORRE", "MUCHO"]),
        ("a b c", 2, &[]),
        ("", 1, &[]),
        ("  sol   y   luna  ", 1, &["SOL", "Y", "LUNA"]),
        ("año camión pingüino", 6, &["CAMIÓN", "PINGÜINO"]),
    ];
    let mut fallos = 0;
    for (texto, minimo, esperado) in casos {
        let r = palabras_largas(texto, minimo);
        if r == esperado {
            println!("OK: palabras_largas({texto:?}, {minimo}) = {r:?}");
        } else {
            println!("FALLO: palabras_largas({texto:?}, {minimo}) = {r:?}, se esperaba {esperado:?}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
