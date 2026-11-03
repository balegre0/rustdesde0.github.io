fn main() {
    // 1. From: conversiones que nunca pierden nada
    let a: u64 = u64::from(200u8);
    println!("u64::from(200u8) = {a}");

    // 2. try_from: conversiones que pueden fallar -> Result
    println!("{:?}", u8::try_from(200i32));
    println!("{:?}", u8::try_from(300i32));
    println!("{:?}", u32::try_from(-1i32));
}
