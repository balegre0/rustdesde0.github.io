use std::num::{IntErrorKind, NonZero};

fn main() {
    println!("{:?}", "42".parse::<i32>());
    println!("{:?}", "-7".parse::<u32>());
    println!("{:?}", "".parse::<i32>());
    println!("{:?}", "3.5".parse::<f64>());
    match "999".parse::<u8>() {
        Err(e) if *e.kind() == IntErrorKind::PosOverflow => println!("demasiado grande: {e}"),
        otro => println!("{otro:?}"),
    }
    println!("{:?}", i64::from_str_radix("ff", 16));
    // NonZero: un entero que nunca es 0 (y Option<NonZero> no ocupa más)
    let n = NonZero::new(5u32);
    let cero = NonZero::new(0u32);
    println!("{n:?} {cero:?}");
    println!("{} {}", size_of::<Option<NonZero<u32>>>(), size_of::<Option<u32>>());
    println!("{} {}", 0.1 + 0.2 == 0.3, (0.1f64 + 0.2 - 0.3).abs() < f64::EPSILON);
}
