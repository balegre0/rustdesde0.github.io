// ⚠ UB a propósito: afirmar que hay un entero donde no se escribió nada.
use std::mem::MaybeUninit;

fn main() {
    let x: MaybeUninit<u32> = MaybeUninit::uninit();
    let n = unsafe { x.assume_init() }; // ¡MAL! nadie escribió x
    if n == 0 {
        println!("cero");
    } else {
        println!("no cero");
    }
}
