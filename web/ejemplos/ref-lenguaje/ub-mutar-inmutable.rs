// Mutar bytes inmutables (aquí, los de un `static` sin UnsafeCell) es UB.
static NUMERO: u32 = 1;

const fn intento() -> u32 {
    let p = &raw const NUMERO as *mut u32;
    unsafe { *p = 2 };
    0
}
const MALO: u32 = intento();

fn main() {
    println!("{MALO} {NUMERO}");
}
