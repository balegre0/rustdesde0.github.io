fn main() {
    let mut fallos = 0;
    let casos = [
        (json_simple("Ana", 30), r#"{"nombre": "Ana", "edad": 30}"#),
        (json_simple("Íñigo", 7), r#"{"nombre": "Íñigo", "edad": 7}"#),
        (json_simple("", 0), r#"{"nombre": "", "edad": 0}"#),
    ];
    for (obtenido, esperado) in casos {
        if obtenido == esperado {
            println!("OK: {obtenido}");
        } else {
            println!("FALLO: esperaba {esperado}, obtuve {obtenido}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
