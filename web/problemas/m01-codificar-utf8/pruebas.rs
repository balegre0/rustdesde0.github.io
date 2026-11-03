fn main() {
    let mut fallos = 0;
    for c in ['a', 'ñ', 'Ñ', '€', 'ü', '👋', '\u{7F}', '\u{80}', '\u{7FF}', '\u{800}', '\u{FFFF}', '\u{10000}'] {
        let mut buf = [0u8; 4];
        let esperado = c.encode_utf8(&mut buf).as_bytes().to_vec();
        let obtenido = a_utf8(c);
        if obtenido == esperado {
            println!("OK: U+{:04X} -> {:02X?}", c as u32, obtenido);
        } else {
            println!("FALLO: U+{:04X}: esperaba {:02X?}, obtuve {:02X?}", c as u32, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
