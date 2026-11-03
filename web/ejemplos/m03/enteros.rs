fn main() {
    // 1. Enteros sin signo (u = unsigned): solo 0 y positivos
    println!("u8:   {} ..= {}", u8::MIN, u8::MAX);
    println!("u16:  {} ..= {}", u16::MIN, u16::MAX);
    println!("u32:  {} ..= {}", u32::MIN, u32::MAX);
    println!("u64:  {} ..= {}", u64::MIN, u64::MAX);

    // 2. Enteros con signo (i = integer): complemento a dos
    println!("i8:   {} ..= {}", i8::MIN, i8::MAX);
    println!("i16:  {} ..= {}", i16::MIN, i16::MAX);
    println!("i32:  {} ..= {}", i32::MIN, i32::MAX);
    println!("i64:  {} ..= {}", i64::MIN, i64::MAX);

    // 3. Los de 128 bits también existen
    println!("u128: {}", u128::MAX);
}
