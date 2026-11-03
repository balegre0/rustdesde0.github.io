#[allow(dead_code)]
struct Normal {
    a: u8,
    b: u32,
    c: u8,
}

#[allow(dead_code)]
#[repr(C)]
struct ComoC {
    a: u8,
    b: u32,
    c: u8,
}

#[allow(dead_code)]
#[repr(C, packed)]
struct Empaquetado {
    a: u8,
    b: u32,
    c: u8,
}

#[allow(dead_code)]
#[repr(align(16))]
struct Alineado(u8);

#[repr(transparent)]
struct Metros(f64);

#[allow(dead_code)]
#[repr(u8)]
enum Color {
    Rojo = 1,
    Verde = 2,
    Azul = 4,
}

fn main() {
    println!("Normal:      {} bytes (Rust reordena los campos)", size_of::<Normal>());
    println!("repr(C):     {} bytes (orden escrito, con relleno)", size_of::<ComoC>());
    println!("packed:      {} bytes (sin relleno)", size_of::<Empaquetado>());
    println!("align(16):   {} bytes, alineación {}", size_of::<Alineado>(), align_of::<Alineado>());
    println!("transparent: {} bytes, igual que f64: {}", size_of::<Metros>(), size_of::<f64>());
    println!("repr(u8):    {} byte; Azul = {}", size_of::<Color>(), Color::Azul as u8);
    let m = Metros(3.5);
    println!("{}", m.0);
}
