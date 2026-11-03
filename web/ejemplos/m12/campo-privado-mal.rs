mod banco {
    pub struct Cuenta {
        pub titular: String,
        saldo: u64,
    }

    impl Cuenta {
        pub fn nueva(titular: &str) -> Cuenta {
            Cuenta { titular: titular.to_string(), saldo: 0 }
        }
    }
}

fn main() {
    let mut c = banco::Cuenta::nueva("Ana");
    c.saldo = 1_000_000;
    println!("{}", c.titular);
}
