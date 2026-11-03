fn main() {
    let mut fallos = 0;
    let t = tabla(7);
    let lineas: Vec<&str> = t.lines().collect();
    let casos = [
        ("10 líneas", lineas.len() == 10),
        ("primera línea", lineas.first() == Some(&" 7 x  1 =   7")),
        ("quinta línea", lineas.get(4) == Some(&" 7 x  5 =  35")),
        ("última línea", lineas.last() == Some(&" 7 x 10 =  70")),
        ("termina en salto", t.ends_with('\n')),
        ("tabla del 12", tabla(12).lines().nth(9) == Some("12 x 10 = 120")),
    ];
    for (que, ok) in casos {
        if ok {
            println!("OK: {que}");
        } else {
            println!("FALLO: {que}");
            fallos += 1;
        }
    }
    if fallos > 0 {
        println!("Tu salida:\n{t}");
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
