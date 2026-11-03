fn main() {
    let mut tabla1 = [None; 16];
    tabla1[0x1] = Some(0);
    tabla1[0xf] = Some(1);
    let mut t2a = [None; 16];
    t2a[0x2] = Some(7);
    let mut t2b = [None; 16];
    t2b[0xf] = Some(300);
    let tablas2 = [t2a, t2b];
    let casos = [
        (0x1234u16, Ok(7 * 256 + 0x34)),
        (0x12ff, Ok(7 * 256 + 0xff)),
        (0xff00, Ok(300 * 256)),
        (0x2000, Err(Fallo::Nivel1)),
        (0x1300, Err(Fallo::Nivel2)),
        (0xfe10, Err(Fallo::Nivel2)),
    ];
    let mut fallos = 0;
    for (v, esperado) in casos {
        let obtenido = traducir(v, &tabla1, &tablas2);
        if obtenido == esperado {
            println!("OK: {v:#06x} -> {:?}", obtenido);
        } else {
            println!("FALLO: {v:#06x}: esperaba {:?}, obtuve {:?}", esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
