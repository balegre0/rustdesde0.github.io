use std::mem::{align_of, size_of};

// 1. Tres campos de tamaños distintos, declarados en este orden.
struct Mezcla {
    a: u8,  // 1 byte
    b: u32, // 4 bytes
    c: u16, // 2 bytes
}

// 2. Los mismos campos, pero pidiendo el orden «tal cual lo escribí».
#[repr(C)]
struct MezclaEnOrden {
    a: u8,
    b: u32,
    c: u16,
}

fn main() {
    println!("Mezcla:        {} bytes (alineamiento {})", size_of::<Mezcla>(), align_of::<Mezcla>());
    println!("MezclaEnOrden: {} bytes (alineamiento {})", size_of::<MezclaEnOrden>(), align_of::<MezclaEnOrden>());
    // 3. Usamos los campos para que el compilador no avise de que sobran.
    let m = Mezcla { a: 1, b: 2, c: 3 };
    let n = MezclaEnOrden { a: 1, b: 2, c: 3 };
    println!("{}", m.a as u32 + m.b + m.c as u32 + n.a as u32 + n.b + n.c as u32);
}
