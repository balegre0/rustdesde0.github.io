// Cómo se documenta (y se audita) una función unsafe.

/// Devuelve el elemento `i` sin comprobar el índice.
///
/// # Safety
///
/// El llamante debe garantizar que `i < v.len()`.
unsafe fn tomar_sin_comprobar(v: &[u32], i: usize) -> u32 {
    debug_assert!(i < v.len(), "precondición rota: i fuera de rango");
    // SAFETY: el contrato de esta función exige i < v.len().
    unsafe { *v.get_unchecked(i) }
}

/// Versión segura: comprueba y luego delega.
fn ultimo(v: &[u32]) -> Option<u32> {
    if v.is_empty() {
        return None;
    }
    // SAFETY: v no está vacío, así que v.len() - 1 < v.len().
    Some(unsafe { tomar_sin_comprobar(v, v.len() - 1) })
}

fn main() {
    println!("{:?}", ultimo(&[3, 1, 4]));
    println!("{:?}", ultimo(&[]));
}
