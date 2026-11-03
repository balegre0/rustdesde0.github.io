// Producir un valor inválido es UB. Las APIs seguras te impiden hacerlo.
use std::num::NonZeroU32;

fn a_bool(b: u8) -> Option<bool> {
    match b {
        0 => Some(false),
        1 => Some(true),
        _ => None, // 2..=255 NO son bools válidos
    }
}

fn main() {
    // 1. bool: solo 0 o 1.
    println!("a_bool(1) = {:?}, a_bool(2) = {:?}", a_bool(1), a_bool(2));

    // 2. char: un valor Unicode escalar (no 0xD800..=0xDFFF, no > 0x10FFFF).
    println!("from_u32(0x00F1) = {:?}", char::from_u32(0x00F1));
    println!("from_u32(0xD800) = {:?}", char::from_u32(0xD800));
    println!("from_u32(0x110000) = {:?}", char::from_u32(0x110000));

    // 3. NonZeroU32 nunca vale 0... y Option lo aprovecha (nicho).
    println!("NonZeroU32::new(0) = {:?}", NonZeroU32::new(0));
    println!("size_of Option<NonZeroU32> = {}", size_of::<Option<NonZeroU32>>());
    println!("size_of Option<u32>        = {}", size_of::<Option<u32>>());

    // 4. Referencias: nunca nulas. Option<&T> usa el nulo como None.
    println!("size_of Option<&u8> = {} = size_of &u8 = {}", size_of::<Option<&u8>>(), size_of::<&u8>());

    // 5. str: siempre UTF-8. from_utf8 lo comprueba.
    println!("{:?}", std::str::from_utf8(&[0x61, 0xC3, 0xB1]));
    let malo = vec![0xFFu8];
    println!("{:?}", std::str::from_utf8(&malo).is_err());
}
