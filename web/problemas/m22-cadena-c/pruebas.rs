fn main() {
    let casos: [(&str, Result<Vec<u8>, usize>); 4] = [
        ("hola", Ok(vec![104, 111, 108, 97, 0])),
        ("", Ok(vec![0])),
        ("ñ", Ok(vec![0xc3, 0xb1, 0])),
        ("ab\0cd", Err(2)),
    ];
    let mut fallos = 0;
    for (s, esperado) in casos {
        let obtenido = a_bytes_c(s);
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", s, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", s, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
