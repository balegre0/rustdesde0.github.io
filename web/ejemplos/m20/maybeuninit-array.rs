// Inicializar un array elemento a elemento con MaybeUninit.
use std::mem::MaybeUninit;

fn cuadrados<const N: usize>() -> [u64; N] {
    // 1. Memoria reservada pero SIN inicializar (y el tipo lo sabe).
    let mut a: [MaybeUninit<u64>; N] = [const { MaybeUninit::uninit() }; N];
    // 2. Escribir cada casilla: write() es seguro.
    for (i, casilla) in a.iter_mut().enumerate() {
        casilla.write((i as u64) * (i as u64));
    }
    // 3. Afirmar que TODO está inicializado: eso sí es unsafe.
    // SAFETY: el bucle escribió las N casillas.
    a.map(|c| unsafe { c.assume_init() })
}

fn main() {
    let c: [u64; 6] = cuadrados();
    println!("{c:?}");
    println!("size_of MaybeUninit<u64> = {}", size_of::<MaybeUninit<u64>>());
}
