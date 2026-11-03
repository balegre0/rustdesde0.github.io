use std::mem::size_of;

// Cuántos bytes ocupa cada tipo. Un byte son 8 bits.
fn main() {
    // 1. Números enteros: el número del nombre son bits
    println!("u8   -> {} byte", size_of::<u8>());
    println!("u16  -> {} bytes", size_of::<u16>());
    println!("u32  -> {} bytes", size_of::<u32>());
    println!("u64  -> {} bytes", size_of::<u64>());
    println!("u128 -> {} bytes", size_of::<u128>());

    // 2. Otros tipos
    println!("bool -> {} byte", size_of::<bool>());
    println!("char -> {} bytes", size_of::<char>());
    println!("f64  -> {} bytes", size_of::<f64>());
}
