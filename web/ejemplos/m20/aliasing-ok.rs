// La versión correcta: respeta el orden de los préstamos.
fn main() {
    let mut x = 0u8;
    let p = &raw mut x;          // 1. puntero crudo hacia x
    let r = unsafe { &mut *p };  // 2. préstamo exclusivo creado desde p
    *r = 2;                      // 3. usar r mientras es el más reciente
    unsafe { *p += 1 };          // 4. volver a p: r ya no se usa nunca más
    println!("{x}");
}
