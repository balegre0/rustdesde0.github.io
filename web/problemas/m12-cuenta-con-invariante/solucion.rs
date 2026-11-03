mod banco {
    pub struct Cuenta {
        saldo: u64,
    }

    impl Cuenta {
        pub fn nueva() -> Cuenta {
            Cuenta { saldo: 0 }
        }

        pub fn ingresar(&mut self, n: u64) {
            self.saldo += n;
        }

        pub fn retirar(&mut self, n: u64) -> Result<(), String> {
            match self.saldo.checked_sub(n) {
                Some(resto) => {
                    self.saldo = resto;
                    Ok(())
                }
                None => Err(format!("saldo insuficiente: hay {}, pides {n}", self.saldo)),
            }
        }

        pub fn saldo(&self) -> u64 {
            self.saldo
        }
    }
}
