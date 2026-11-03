// Un volcado de memoria: 12 bytes sin etiqueta. ¿Qué significan?
fn main() {
    let memoria: [u8; 12] = [
        0x2c, 0x01, 0x00, 0x00, // bytes 0-3
        0x00, 0x00, 0x48, 0x40, // bytes 4-7
        0x61, 0xc3, 0xb1, 0x6f, // bytes 8-11
    ];

    // 1. El volcado tal cual: posición y byte en hexadecimal
    for (posicion, byte) in memoria.iter().enumerate() {
        println!("{posicion:>2}: {:02x} = {:08b}", byte, byte);
    }

    // 2. Los mismos 4 primeros bytes, leídos de cuatro maneras
    let cuatro = [memoria[0], memoria[1], memoria[2], memoria[3]];
    println!("u32 le: {}", u32::from_le_bytes(cuatro));
    println!("u32 be: {}", u32::from_be_bytes(cuatro));
    println!("i32 le: {}", i32::from_le_bytes(cuatro));
    println!("f32 le: {:e}", f32::from_le_bytes(cuatro));

    // 3. Los bytes 4-7 tienen pinta de f32 (0x40 al final: signo 0, exponente alto)
    let otros = [memoria[4], memoria[5], memoria[6], memoria[7]];
    println!("bytes 4-7 como f32 le: {}", f32::from_le_bytes(otros));

    // 4. Los bytes 8-11 tienen pinta de texto (c3 empieza un carácter de 2 bytes)
    println!("bytes 8-11 como texto: {:?}", std::str::from_utf8(&memoria[8..12]));
}
