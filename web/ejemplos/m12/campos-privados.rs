mod banco {
    pub struct Cuenta {
        pub titular: String,
        saldo: u64, // privado: solo el módulo `banco` puede tocarlo
    }

    impl Cuenta {
        pub fn nueva(titular: &str) -> Cuenta {
            Cuenta { titular: titular.to_string(), saldo: 0 }
        }

        pub fn ingresar(&mut self, cantidad: u64) {
            self.saldo += cantidad;
        }

        pub fn saldo(&self) -> u64 {
            self.saldo
        }
    }
}

fn main() {
    let mut c = banco::Cuenta::nueva("Ana");
    c.ingresar(50);
    c.ingresar(20);
    println!("{}: {}", c.titular, c.saldo());
}
