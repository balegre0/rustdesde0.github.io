//! Un crate de ejemplo: el comentario con `//!` documenta el módulo que lo contiene.

/// Devuelve el mayor de dos números.
///
/// # Ejemplos
///
/// ```
/// assert_eq!(mayor(3, 7), 7);
/// ```
pub fn mayor(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// Divide `a` entre `b`.
///
/// # Panics
///
/// Entra en pánico si `b` es cero.
pub fn dividir(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    println!("{} {}", mayor(3, 7), dividir(9, 3));
}
