use std::mem::MaybeUninit;

// Un entero debe estar inicializado: leer memoria sin inicializar como u32 es UB.
const MALO: u32 = unsafe { MaybeUninit::<u32>::uninit().assume_init() };

fn main() {
    println!("{MALO}");
}
