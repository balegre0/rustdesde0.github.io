
fn main() {
    let casos: [(&[&str], Result<i64, String>); 6] = [
        (&["1", " 2 ", "3"], Ok(6)),
        (&["1", "dos", "tres"], Err(String::from("«dos» no es un número"))),
        (&[], Ok(0)),
        (&["-5", "5"], Ok(0)),
        (&["9000000000", "1"], Ok(9000000001)),
        (&["4", "4.5"], Err(String::from("«4.5» no es un número"))),
    ];
    let mut fallos = 0;
    for (entrada, esperado) in casos {
        let obtenido = sumar(entrada);
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", entrada, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", entrada, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
