
fn main() {
    let c = |saldo: u64, bloqueada: bool| Cuenta { saldo, bloqueada };
    // (origen, destino, cantidad, resultado esperado, origen después, destino después)
    let casos = [
        (c(100, false), c(0, false), 30, Ok(()), 70, 30),
        (c(100, false), c(5, false), 100, Ok(()), 0, 105),
        (c(100, false), c(0, false), 0, Err(ErrorBanco::CantidadCero), 100, 0),
        (c(100, true), c(0, false), 10, Err(ErrorBanco::Bloqueada), 100, 0),
        (c(40, false), c(0, false), 50, Err(ErrorBanco::SaldoInsuficiente { falta: 10 }), 40, 0),
        (c(10, true), c(0, false), 50, Err(ErrorBanco::Bloqueada), 10, 0),
        (c(10, false), c(u64::MAX - 5, false), 10, Err(ErrorBanco::Desbordamiento), 10, u64::MAX - 5),
    ];
    let mut fallos = 0;
    for (i, (mut origen, mut destino, cantidad, esperado, so, sd)) in casos.into_iter().enumerate() {
        let obtenido = transferir(&mut origen, &mut destino, cantidad);
        if obtenido == esperado && origen.saldo == so && destino.saldo == sd {
            println!("OK: caso {i}: {:?}, saldos {} y {}", obtenido, origen.saldo, destino.saldo);
        } else {
            println!(
                "FALLO: caso {i}: esperaba {:?} con saldos {so} y {sd}; obtuve {:?} con saldos {} y {}",
                esperado, obtenido, origen.saldo, destino.saldo
            );
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
