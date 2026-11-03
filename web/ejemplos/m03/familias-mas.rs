fn main() {
    // También existen para resta, multiplicación, división...
    println!("{:?}", 100i8.checked_mul(2));
    println!("{:?}", 10u32.checked_div(0));
    println!("{}", i8::MIN.wrapping_neg());
    println!("{:?}", i8::MIN.checked_neg());
    println!("{}", (-100i8).saturating_sub(100));
    // Y el valor absoluto también desborda en i8::MIN
    println!("{:?}", i8::MIN.checked_abs());
}
