// ⚠ UB a propósito: un préstamo exclusivo invalidado por un puntero hermano.
// rustc lo compila sin quejarse. Pásalo por Miri (Tools → Miri).
fn main() {
    let mut x = 0u8;
    let p = &raw mut x;          // 1. puntero crudo hacia x
    let r = unsafe { &mut *p };  // 2. préstamo exclusivo creado desde p
    unsafe { *p = 1 };           // 3. escribir por p: r deja de ser válido
    *r = 2;                      // 4. usar r: UB (Stacked y Tree Borrows)
    println!("{x}");
}
