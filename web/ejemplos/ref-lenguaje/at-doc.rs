//! Documentación del crate.

/// Suma dos números. (`///` es lo mismo que `#[doc = "…"]`.)
#[doc(alias = "add")] // el buscador de rustdoc la encuentra también por «add»
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[doc = "Resta dos números, escrito con el atributo directamente."]
pub fn restar(a: i32, b: i32) -> i32 {
    a - b
}

#[doc(hidden)] // pública, pero no aparece en la documentación
pub fn interna() -> i32 {
    0
}

fn main() {
    println!("{} {} {}", sumar(2, 3), restar(5, 1), interna());
}
