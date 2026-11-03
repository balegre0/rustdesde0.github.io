use std::num::NonZeroU32;

#[allow(dead_code)]
struct Rust { a: u8, b: u32, c: u16 }       // el compilador puede reordenar
#[allow(dead_code)]
#[repr(C)]
struct C { a: u8, b: u32, c: u16 }          // orden de declaración + relleno
#[allow(dead_code)]
#[repr(C, packed)]
struct Empaquetado { a: u8, b: u32, c: u16 } // sin relleno, alineación 1

macro_rules! ver {
    ($($t:ty),* $(,)?) => {
        $( println!("{:<22} tamaño {:>2}  alineación {}", stringify!($t), size_of::<$t>(), align_of::<$t>()); )*
    };
}

fn main() {
    ver!(bool, char, u8, u32, u64, Rust, C, Empaquetado);
    // Nichos: el compilador guarda la etiqueta del Option en valores imposibles.
    ver!(Option<bool>, Option<u32>, Option<NonZeroU32>, Option<char>);
    // Punteros: el tamaño depende de la plataforma (aquí, la que ejecuta el ejemplo).
    let p = size_of::<usize>();
    println!("&u8 = {} punteros, &[u8] = {}, &str = {}, Box<u8> = {}, Option<&u8> = {}",
        size_of::<&u8>() / p, size_of::<&[u8]>() / p, size_of::<&str>() / p,
        size_of::<Box<u8>>() / p, size_of::<Option<&u8>>() / p);
    println!("Option<*const u8> = {} punteros (puede ser nulo: sin nicho)", size_of::<Option<*const u8>>() / p);
}
