use std::mem::{size_of, size_of_val, align_of};

struct Punto {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
struct Mezcla {
    a: u8,
    b: u32,
    c: u8,
}

fn main() {
    println!("u8 {}  u32 {}  u64 {}  f64 {}", size_of::<u8>(), size_of::<u32>(), size_of::<u64>(), size_of::<f64>());
    println!("bool {}  char {}  ()  {}", size_of::<bool>(), size_of::<char>(), size_of::<()>());
    println!("usize {}  &u8 {}  Box<u8> {}", size_of::<usize>(), size_of::<&u8>(), size_of::<Box<u8>>());
    println!("&str {}  String {}", size_of::<&str>(), size_of::<String>());
    println!("Punto {}  Mezcla {} (alineación {})", size_of::<Punto>(), size_of::<Mezcla>(), align_of::<Mezcla>());
    // nicho: Option<&T> no ocupa más que &T
    println!("Option<&u8> {}  Option<u32> {}", size_of::<Option<&u8>>(), size_of::<Option<u32>>());
    let v = [0u16; 10];
    println!("array {}", size_of_val(&v));
    let _ = Punto { x: 0, y: 0 }.x + Punto { x: 0, y: 0 }.y;
}
