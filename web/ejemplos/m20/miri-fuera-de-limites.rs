// ⚠ UB a propósito: leer una posición más allá del final. Pásalo por Miri.
fn main() {
    let v = [1u8, 2, 3];
    let p = v.as_ptr();
    let mut suma = 0u32;
    for i in 0..=3 {               // 1. ¡error de uno! 0..=3 son 4 vueltas
        suma += unsafe { *p.add(i) } as u32; // 2. en i = 3: fuera de límites
    }
    println!("{suma}");
}
