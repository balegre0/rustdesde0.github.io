// Edición 2024: el cuerpo de una `unsafe fn` ya NO es un bloque unsafe implícito.
unsafe fn primero(p: *const u8) -> u8 {
    *p // operación insegura sin su propio `unsafe {}`
}

fn main() {
    let x = 1u8;
    // SAFETY: &x es válido.
    let v = unsafe { primero(&x) };
    println!("{v}");
}
