fn main() {
    let n: u32 = 0x1234_5678;
    for byte in n.to_le_bytes() {
        print!("{:02x} ", byte);
    }
    println!();
}
