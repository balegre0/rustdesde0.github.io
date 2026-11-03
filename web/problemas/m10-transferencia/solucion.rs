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
    if cantidad == 0 {
        return Err(ErrorBanco::CantidadCero);
    }
    if origen.bloqueada {
        return Err(ErrorBanco::Bloqueada);
    }
    if origen.saldo < cantidad {
        return Err(ErrorBanco::SaldoInsuficiente { falta: cantidad - origen.saldo });
    }
    let nuevo_destino = destino.saldo.checked_add(cantidad).ok_or(ErrorBanco::Desbordamiento)?;
    origen.saldo -= cantidad;
    destino.saldo = nuevo_destino;
    Ok(())
}
