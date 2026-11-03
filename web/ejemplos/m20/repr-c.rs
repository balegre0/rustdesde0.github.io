// repr(C) fija el orden y el relleno de los campos (lo que espera la ABI de C).
#[repr(C)]
struct EnOrdenC {
    a: u8,  // byte 0, luego 3 de relleno
    b: u32, // bytes 4..8
    c: u16, // bytes 8..10, luego 2 de relleno
}

struct LibreRust {
    a: u8,
    b: u32,
    c: u16,
}

fn main() {
    println!("repr(C):    {} bytes", size_of::<EnOrdenC>());
    println!("repr(Rust): {} bytes", size_of::<LibreRust>());
    println!("offsets C: a={} b={} c={}",
        std::mem::offset_of!(EnOrdenC, a),
        std::mem::offset_of!(EnOrdenC, b),
        std::mem::offset_of!(EnOrdenC, c));
    let x = LibreRust { a: 1, b: 2, c: 3 };
    let y = EnOrdenC { a: 1, b: 2, c: 3 };
    println!("{} {}", x.a as u32 + x.b + x.c as u32, y.a as u32 + y.b + y.c as u32);
}
