fn main() {
    let mut fallos = 0;
    let compra = [("pan", 2, 1.25), ("leche", 3, 0.95), ("aceite", 1, 8.05)];
    let casos = [
        (linea_ticket("pan", 2, 1.25), "pan          x  2     2.50 €"),
        (linea_ticket("aceite", 1, 8.05), "aceite       x  1     8.05 €"),
        (linea_ticket("leche", 12, 0.95), "leche        x 12    11.40 €"),
        (total(&compra), "TOTAL                13.40 €"),
    ];
    for (obtenido, esperado) in casos {
        if obtenido == esperado {
            println!("OK: «{obtenido}»");
        } else {
            println!("FALLO: esperaba «{esperado}»");
            println!("       obtuve   «{obtenido}»");
            fallos += 1;
        }
    }
    let ancho = |s: &str| s.chars().count();
    if ancho(&total(&compra)) != ancho(&linea_ticket("pan", 2, 1.25)) {
        println!("FALLO: la línea del total no mide lo mismo que las demás");
        fallos += 1;
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
