// La ficha de un jugador, guardada por un PC little-endian:
//   bytes 0-3:  puntos  (u32)
//   bytes 4-7:  altura  (f32, en metros)
//   bytes 8-11: nombre  (texto UTF-8)
// El programa la lee MAL. Arréglalo para que imprima:
// puntos: 300 | altura: 1.75 | nombre: año
fn main() {
    let memoria: [u8; 12] = [
        0x2c, 0x01, 0x00, 0x00,
        0x00, 0x00, 0xe0, 0x3f,
        0x61, 0xc3, 0xb1, 0x6f,
    ];

    // 1. Los números se guardaron en little-endian: se leen con from_le_bytes
    let puntos = u32::from_le_bytes([memoria[0], memoria[1], memoria[2], memoria[3]]);
    let altura = f32::from_le_bytes([memoria[4], memoria[5], memoria[6], memoria[7]]);
    // 2. El nombre ocupa los bytes 8, 9, 10 y 11: el rango es 8..12 («ñ» ocupa 2 bytes)
    let nombre = std::str::from_utf8(&memoria[8..12]).unwrap_or("¿texto roto?");

    println!("puntos: {puntos} | altura: {altura} | nombre: {nombre}");
}
