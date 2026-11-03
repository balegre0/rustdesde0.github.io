//! # Saludos
//!
//! Un programa diminuto para practicar la documentación.

/// Escribe un saludo para `nombre`.
///
/// Los comentarios con tres barras documentan lo que viene debajo.
fn saludar(nombre: &str) {
    println!("¡Hola, {nombre}!");
}

fn main() {
    saludar("Ana");
    saludar("Luis");
}
