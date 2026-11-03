mod banco {
    pub struct Cuenta {
        saldo: u64,
    }

    impl Cuenta {
        pub fn nueva() -> Cuenta {
            Cuenta { saldo: 0 }
        }

        pub fn ingresar(&mut self, n: u64) {
            todo!()
        }

        pub fn retirar(&mut self, n: u64) -> Result<(), String> {
            todo!()
        }

        pub fn saldo(&self) -> u64 {
            self.saldo
        }
    }
}
