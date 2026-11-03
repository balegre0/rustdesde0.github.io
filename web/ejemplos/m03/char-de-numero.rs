fn main() {
    // 1. bool como número: false = 0, true = 1
    println!("{} {}", false as u8, true as u8);

    // 2. De número a char: puede fallar -> Option
    println!("{:?}", char::from_u32(0x41));
    println!("{:?}", char::from_u32(0x1F980));
    // 3. Los "sustitutos" D800..DFFF no son valores escalares Unicode
    println!("{:?}", char::from_u32(0xD800));
    // 4. Ni nada por encima de 10FFFF
    println!("{:?}", char::from_u32(0x110000));
    println!("char::MAX = U+{:X}", char::MAX as u32);
}
