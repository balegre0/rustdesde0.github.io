// Una cifra hexadecimal resume 4 bits. Dos cifras, un byte entero.
fn main() {
    // 1. Las 16 cifras: 0-9 y a-f
    for n in [0, 9, 10, 15] {
        println!("{n:>3} -> {:04b} -> {:x}", n, n);
    }

    // 2. Un byte se escribe siempre con 2 cifras: {:02x}
    for byte in [5u8, 16, 200, 255] {
        println!("{byte:>3} -> {:08b} -> {:02x}", byte, byte);
    }

    // 3. Con # delante, Rust añade el prefijo 0x
    println!("{:#x}", 4096);

    // 4. Y puedes escribir números en hexadecimal con 0x
    let n = 0x2a;
    println!("0x2a vale {n}");
}
