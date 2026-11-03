// Un u32 son 4 bytes. ¿En qué orden se guardan?
fn main() {
    let n: u32 = 0x1234_5678;

    // 1. Little-endian: primero el byte menos significativo
    println!("le: {:02x?}", n.to_le_bytes());

    // 2. Big-endian: primero el byte más significativo (como lo escribimos)
    println!("be: {:02x?}", n.to_be_bytes());

    // 3. ¿Y esta máquina? to_ne_bytes usa el orden nativo
    println!("ne: {:02x?}", n.to_ne_bytes());

    // 4. De bytes a número: hay que saber en qué orden venían
    let bytes = [0x2c, 0x01, 0x00, 0x00];
    println!("leídos como le: {}", u32::from_le_bytes(bytes));
    println!("leídos como be: {}", u32::from_be_bytes(bytes));
}
