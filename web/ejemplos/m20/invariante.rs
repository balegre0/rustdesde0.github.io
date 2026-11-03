// Invariante de seguridad protegida por la privacidad.
mod ascii {
    /// Texto que SOLO contiene bytes ASCII (invariante de seguridad).
    pub struct Ascii {
        bytes: Vec<u8>, // privado: nadie de fuera puede romper la invariante
    }

    impl Ascii {
        pub fn new(s: &str) -> Option<Ascii> {
            if s.is_ascii() { Some(Ascii { bytes: s.as_bytes().to_vec() }) } else { None }
        }

        pub fn como_str(&self) -> &str {
            // SAFETY: la invariante (todo ASCII) implica UTF-8 válido, y solo
            // `new` crea valores, así que la invariante se cumple siempre.
            unsafe { std::str::from_utf8_unchecked(&self.bytes) }
        }

        // Si existiera esto, como_str sería unsound, sin escribir
        // ni un unsafe más:
        // pub fn bytes_mut(&mut self) -> &mut Vec<u8> { &mut self.bytes }
    }
}

fn main() {
    let a = ascii::Ascii::new("Hola").unwrap();
    println!("{}", a.como_str());
    println!("{}", ascii::Ascii::new("año").is_none());
}
