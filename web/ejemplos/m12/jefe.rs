mod biblioteca {
    pub mod libros {
        pub struct Libro {
            pub titulo: String,
            prestado: bool,
        }

        impl Libro {
            pub fn nuevo(titulo: &str) -> Libro {
                Libro { titulo: titulo.to_string(), prestado: false }
            }

            pub fn esta_prestado(&self) -> bool {
                self.prestado
            }

            pub(crate) fn marcar(&mut self, prestado: bool) {
                self.prestado = prestado;
            }
        }
    }

    pub mod prestamos {
        use super::libros::Libro;

        pub fn prestar(libro: &mut Libro) -> Result<(), String> {
            if libro.esta_prestado() {
                return Err(format!("«{}» ya está prestado", libro.titulo));
            }
            libro.marcar(true);
            Ok(())
        }
    }

    pub use libros::Libro;
    pub use prestamos::prestar;
}

use biblioteca::{prestar, Libro};

fn main() {
    let mut l = Libro::nuevo("Rayuela");
    println!("{:?}", prestar(&mut l));
    println!("{:?}", prestar(&mut l));
    println!("prestado: {}", l.esta_prestado());
}
