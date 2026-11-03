use std::mem::{align_of, size_of};
use std::num::NonZeroU32;

#[allow(dead_code)]
struct Desordenado { a: u8, b: u32, c: u16 }

#[allow(dead_code)]
#[repr(C)]
struct EnOrden { a: u8, b: u32, c: u16 }

#[allow(dead_code)]
enum DosCasos { A(Box<u64>), B }

#[allow(dead_code)]
enum TresCasos { A(Box<u64>), B, C }

macro_rules! fila {
    ($t:ty) => {
        println!("{:<26} tamaño {:>2}  alineamiento {}", stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

fn main() {
    // 1. Primitivos
    fila!(bool);
    fila!(char);
    fila!(u128);
    fila!(usize);
    // 2. Nichos: el valor «imposible» guarda el None
    fila!(&u8);
    fila!(Option<&u8>);
    fila!(Option<*const u8>);
    fila!(Option<u32>);
    fila!(Option<NonZeroU32>);
    fila!(Option<bool>);
    fila!(Option<Option<bool>>);
    fila!(Option<char>);
    // 3. Reordenación de campos
    fila!(Desordenado);
    fila!(EnOrden);
    // 4. Un puntero aporta UN solo nicho
    fila!(DosCasos);
    fila!(TresCasos);
    // 5. Contenedores
    fila!(Vec<u8>);
    fila!(Option<Vec<u8>>);
    fila!(&str);
    fila!(&dyn std::fmt::Debug);
}
