// Organiza el código: un módulo `biblioteca` con dos submódulos, `libros` y `prestamos`.
// 1. `Libro` tiene `titulo` público y `prestado` PRIVADO.
// 2. `prestamos::prestar` usa `super::libros::Libro`.
// 3. `biblioteca` re-exporta `Libro` y `prestar` con `pub use`, para que main pueda
//    escribir `use biblioteca::{prestar, Libro};`.
// Cambia solo la visibilidad (`pub`, `pub(crate)`) y los `use` que falten.
mod biblioteca {
    mod libros {
        struct Libro {
            titulo: String,
            prestado: bool,
        }

        impl Libro {
            fn nuevo(titulo: &str) -> Libro {
                Libro { titulo: titulo.to_string(), prestado: false }
            }

            fn esta_prestado(&self) -> bool {
                self.prestado
            }

            fn marcar(&mut self, prestado: bool) {
                self.prestado = prestado;
            }
        }
    }

    mod prestamos {
        fn prestar(libro: &mut Libro) -> Result<(), String> {
            if libro.esta_prestado() {
                return Err(format!("«{}» ya está prestado", libro.titulo));
            }
            libro.marcar(true);
            Ok(())
        }
    }
}

use biblioteca::{prestar, Libro};

fn main() {
    let mut l = Libro::nuevo("Rayuela");
    println!("{:?}", prestar(&mut l));
    println!("{:?}", prestar(&mut l));
    println!("prestado: {}", l.esta_prestado());
}
