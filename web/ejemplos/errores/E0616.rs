mod cuenta {
    pub struct Cuenta {
        saldo: u32,
    }
    impl Cuenta {
        pub fn nueva() -> Cuenta {
            Cuenta { saldo: 0 }
        }
    }
}

fn main() {
    let c = cuenta::Cuenta::nueva();
    println!("{}", c.saldo);
}
