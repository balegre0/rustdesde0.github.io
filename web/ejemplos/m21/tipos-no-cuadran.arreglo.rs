fn area(lado: u32) -> u32 {
    lado * lado
}

fn main() {
    let lado: u8 = 12;
    // Conversión explícita y sin pérdida: u8 -> u32
    println!("{}", area(u32::from(lado)));
}
