//! Un pequeño conversor de temperaturas, documentado como lo haría
//! una biblioteca de verdad.

/// Pasa grados Celsius a Fahrenheit.
///
/// # Ejemplos
///
/// ```
/// assert_eq!(a_fahrenheit(100.0), 212.0);
/// ```
fn a_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// Pasa grados Fahrenheit a Celsius. Es la inversa de [`a_fahrenheit`].
fn a_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Lee una temperatura como `"36.6"`.
///
/// # Errores
///
/// Devuelve `Err` con un mensaje si el texto no es un número.
fn leer(texto: &str) -> Result<f64, String> {
    texto.trim().parse().map_err(|_| format!("«{texto}» no es un número"))
}

fn main() {
    // 1. Los ejemplos de la documentación, ejecutados a mano.
    assert_eq!(a_fahrenheit(100.0), 212.0);
    assert_eq!(a_celsius(212.0), 100.0);
    // 2. Lo que promete la sección «Errores».
    println!("{:?}", leer(" 36.6 "));
    println!("{:?}", leer("caliente"));
}
