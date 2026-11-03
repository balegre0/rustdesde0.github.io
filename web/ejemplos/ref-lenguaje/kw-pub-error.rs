mod cuenta {
    pub struct Cuenta {
        pub titular: String,
        saldo: i64,
    }
    pub fn abrir() -> Cuenta { Cuenta { titular: "Lin".into(), saldo: 0 } }
}

fn main() {
    let mut c = cuenta::abrir();
    // `saldo` es privado: no se puede modificar desde fuera del módulo.
    c.saldo = 1_000_000;
    println!("{}", c.titular);
}
