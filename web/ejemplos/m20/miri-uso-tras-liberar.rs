// ⚠ UB a propósito: leer memoria ya liberada. Pásalo por Miri.
fn main() {
    let caja = Box::new(42);
    let p: *const i32 = &*caja; // 1. puntero crudo hacia el heap
    drop(caja);                 // 2. se libera la memoria
    let v = unsafe { *p };      // 3. uso tras liberar: UB
    println!("{v}");
}
