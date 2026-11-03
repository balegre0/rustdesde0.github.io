fn main() {
    println!("{}", 200u8.saturating_mul(2));
    println!("{}", 16u8.wrapping_mul(17));
    println!("{:?}", 16u8.checked_mul(16));
    println!("{:?}", 100i8.overflowing_add(100));
    println!("{}", (-120i8).saturating_sub(20));
}
