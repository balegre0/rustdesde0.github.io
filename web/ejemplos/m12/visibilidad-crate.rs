mod red {
    pub(crate) fn conectar() -> &'static str {
        interno::saludo()
    }

    mod interno {
        // pub(super): visible solo para el módulo padre (`red`).
        pub(super) fn saludo() -> &'static str {
            "conectado"
        }
    }
}

fn main() {
    println!("{}", red::conectar());
}
