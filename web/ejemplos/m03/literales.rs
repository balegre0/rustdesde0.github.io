fn main() {
    // 1. El guion bajo solo sirve para leer mejor
    let millon = 1_000_000;
    // 2. Otras bases: hexadecimal, binario y octal
    let hex = 0xff;
    let bin = 0b1010;
    let oct = 0o17;
    // 3. Un sufijo fija el tipo del literal
    let pequeño = 5u8;
    let grande = 7_i64;
    println!("{millon} {hex} {bin} {oct} {pequeño} {grande}");
    // 4. Un byte como literal: b'A' es un u8
    let letra = b'A';
    println!("b'A' = {letra}");
}
