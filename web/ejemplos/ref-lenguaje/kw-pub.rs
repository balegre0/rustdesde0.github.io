mod cuenta {
    pub struct Cuenta {
        pub titular: String, // campo público
        saldo: i64,          // campo privado: nadie de fuera puede tocarlo
    }

    impl Cuenta {
        pub fn nueva(titular: &str) -> Self {
            Cuenta { titular: titular.to_string(), saldo: 0 }
        }
        pub fn ingresar(&mut self, c: i64) {
            if c > 0 { self.saldo += c; }
        }
        pub fn saldo(&self) -> i64 { self.saldo }
    }

    pub(super) fn version() -> u32 { 2 }
}

fn main() {
    let mut c = cuenta::Cuenta::nueva("Lin");
    c.ingresar(50);
    c.ingresar(-999); // ignorado: la regla vive dentro del módulo
    println!("{}: {}", c.titular, c.saldo());
    println!("v{}", cuenta::version());
}
