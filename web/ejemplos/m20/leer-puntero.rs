// Completa `leer`: devuelve el valor al que apunta `p`.
/// # Safety
/// `p` debe apuntar a un `u8` válido.
unsafe fn leer(p: *const u8) -> u8 {
    // SAFETY: el llamante garantiza que `p` es válido.
    unsafe { *p }
}

fn main() {
    let datos = [10u8, 20, 30];
    let p = &raw const datos[1];
    // SAFETY: `p` apunta a datos[1], que existe.
    let v = unsafe { leer(p) };
    println!("leído: {v}");
}
