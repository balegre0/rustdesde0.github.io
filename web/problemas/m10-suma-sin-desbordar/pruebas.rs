fn main() {
    let casos: [(&[&str], Result<u8, ErrorSuma>); 6] = [
        (&["1", "2", "3"], Ok(6)),
        (&[], Ok(0)),
        (&["200", "55"], Ok(255)),
        (&["200", "56"], Err(ErrorSuma::Desborda)),
        (&["7", "siete", "8"], Err(ErrorSuma::NoEsNumero(1))),
        (&["300"], Err(ErrorSuma::NoEsNumero(0))),
    ];
    let mut fallos = 0;
    for (entrada, esperado) in casos {
        let obtenido = sumar_todo(entrada);
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
