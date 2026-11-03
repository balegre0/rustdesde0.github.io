fn main() {
    let mut fallos = 0;
    let casos = [
        (Euros(1205), "12,05 €"),
        (Euros(150) + Euros(275), "4,25 €"),
        (Euros(1000) - Euros(1350), "-3,50 €"),
        (Euros(199) * 3, "5,97 €"),
        (Euros(7), "0,07 €"),
        (Euros(0), "0,00 €"),
    ];
    for (e, esperado) in casos {
        let obtenido = e.to_string();
        if obtenido == esperado {
            println!("OK: {:?} -> {obtenido}", e);
        } else {
            println!("FALLO: {:?}: esperaba {esperado:?}, obtuve {obtenido:?}", e);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
