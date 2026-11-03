mod cuenta {
    pub struct Cuenta {
        saldo: u32,
    }
    impl Cuenta {
        pub fn nueva() -> Cuenta {
            Cuenta { saldo: 0 }
        }
        pub fn saldo(&self) -> u32 { // acceso de solo lectura
            self.saldo
        }
    }
}

fn main() {
    let c = cuenta::Cuenta::nueva();
    println!("{}", c.saldo());
}
