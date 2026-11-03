use std::sync::Mutex;

struct Cuenta {
    id: u32,
    saldo: Mutex<u64>,
}

fn transferir(a: &Cuenta, b: &Cuenta, cantidad: u64) -> Result<(), String> {
    todo!()
}
