#[derive(Debug, PartialEq, Clone)]
struct Cuenta {
    saldo: u64,
    bloqueada: bool,
}

#[derive(Debug, PartialEq)]
enum ErrorBanco {
    CantidadCero,
    Bloqueada,
    SaldoInsuficiente { falta: u64 },
    Desbordamiento,
}

fn transferir(origen: &mut Cuenta, destino: &mut Cuenta, cantidad: u64) -> Result<(), ErrorBanco> {
    todo!()
}
