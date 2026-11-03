mod correo {
    pub struct Email {
        texto: String,
    }

    impl Email {
        pub fn parsear(s: &str) -> Result<Email, &'static str> {
            let s = s.trim();
            if s.is_empty() {
                return Err("vacío");
            }
            if s.matches('@').count() != 1 {
                return Err("hace falta una @");
            }
            let (u, d) = s.split_once('@').unwrap_or_default();
            if u.is_empty() {
                return Err("falta el usuario");
            }
            if !d.contains('.') || d.starts_with('.') || d.ends_with('.') {
                return Err("dominio no válido");
            }
            Ok(Email { texto: s.to_lowercase() })
        }

        pub fn usuario(&self) -> &str {
            // El tipo garantiza que hay exactamente una @.
            self.texto.split_once('@').map_or("", |(u, _)| u)
        }

        pub fn dominio(&self) -> &str {
            self.texto.split_once('@').map_or("", |(_, d)| d)
        }
    }
}
