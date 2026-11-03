fn main() {
    let color: u32 = 0x12AB34CD;
    let b0 = color as u8;
    let b1 = (color >> 8) as u8;
    let b2 = (color >> 16) as u8;
    let b3 = (color >> 24) as u8;
    println!("{b0:x} {b1:x} {b2:x} {b3:x}");
}
