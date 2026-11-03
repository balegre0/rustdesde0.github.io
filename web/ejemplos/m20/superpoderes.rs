// Los cinco superpoderes de `unsafe`, uno a uno (todos usados de forma correcta).
use std::sync::atomic::{AtomicU32, Ordering};

static mut VISITAS: u32 = 0; // 3. un `static mut`

/// # Safety
/// `p` debe apuntar a un `i32` válido y alineado.
unsafe fn doblar(p: *const i32) -> i32 {
    // SAFETY: lo exige el contrato de `doblar`.
    unsafe { *p * 2 }
}

union Bits {
    entero: u32,
    real: f32,
}

// 4. implementar un trait `unsafe`
struct Contador(AtomicU32);
// SAFETY: AtomicU32 ya se puede compartir; lo decimos a mano solo para ver la sintaxis.
unsafe impl Sync for Contador {}

fn main() {
    let x = 21;
    let p = &raw const x; // crear un puntero crudo NO necesita unsafe

    // 1. desreferenciar un puntero crudo
    // SAFETY: `p` sale de `x`, que sigue viva y está alineada.
    let v = unsafe { *p };
    println!("1. *p = {v}");

    // 2. llamar a una función `unsafe`
    // SAFETY: `p` apunta a `x`, un i32 válido.
    println!("2. doblar(p) = {}", unsafe { doblar(p) });

    // 3. tocar un `static mut` (sin crear referencias a él)
    // SAFETY: este programa tiene un solo hilo.
    unsafe { VISITAS += 1 };
    let visitas = unsafe { VISITAS };
    println!("3. VISITAS = {visitas}");

    // 4. ya usado arriba: `unsafe impl Sync`
    let c = Contador(AtomicU32::new(7));
    println!("4. contador = {}", c.0.load(Ordering::Relaxed));

    // 5. leer un campo de una `union`
    let b = Bits { real: 1.0 };
    // SAFETY: cualquier patrón de 32 bits es un u32 válido.
    println!("5. bits de 1.0 = {:#010x}", unsafe { b.entero });
}
