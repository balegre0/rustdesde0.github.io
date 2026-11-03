fn main() {
    let casos: [(&str, u32, u32); 8] = [
        ("poner(0, 13)", poner(0, 13), 0x2000),
        ("poner ya puesto", poner(0b100, 2), 0b100),
        ("quitar(0xff, 0)", quitar(0xff, 0), 0xfe),
        ("alternar dos veces", alternar(alternar(0b1010, 1), 1), 0b1010),
        ("alternar(0b1010, 0)", alternar(0b1010, 0), 0b1011),
        ("leer bit 3 de 0b1000", leer(0b1000, 3) as u32, 1),
        ("campo(0b1101_0000, 4, 4)", campo(0b1101_0000, 4, 4), 0b1101),
        ("campo(0xabcd_1234, 16, 8)", campo(0xabcd_1234, 16, 8), 0xcd),
    ];
    let mut fallos = 0;
    for (que, obtenido, esperado) in casos {
        if obtenido == esperado {
            println!("OK: {que} = {obtenido:#x}");
        } else {
            println!("FALLO: {que}: esperaba {esperado:#x}, obtuve {obtenido:#x}");
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
