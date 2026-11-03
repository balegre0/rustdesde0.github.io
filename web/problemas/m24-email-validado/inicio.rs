mod correo {
    pub struct Email {
        texto: String,
    }

    impl Email {
        pub fn parsear(s: &str) -> Result<Email, &'static str> {
            todo!()
        }

        pub fn usuario(&self) -> &str {
            todo!()
        }

        pub fn dominio(&self) -> &str {
            todo!()
        }
    }
}
