fn main() {
    let mut fallos = 0;
    let mut comprobar = |que: &str, obtenido: String, esperado: &str| {
        if obtenido == esperado {
            println!("OK: {que} -> {obtenido}");
        } else {
            println!("FALLO: {que}: esperaba {esperado:?}, obtuve {obtenido:?}");
            fallos += 1;
        }
    };
    let d = |r: Result<Pedido, Pedido>| match r {
        Ok(p) => format!("ok: {}", p.describir()),
        Err(p) => format!("err: {}", p.describir()),
    };
    comprobar("pagar un pendiente", d(Pedido::Pendiente.pagar(30)), "ok: pagado (30 €)");
    comprobar("enviar sin pagar", d(Pedido::Pendiente.enviar("X1")), "err: pendiente");
    let pagado = Pedido::Pendiente.pagar(30).unwrap();
    comprobar("enviar un pagado", d(pagado.clone().enviar("ES-42")), "ok: enviado (ES-42)");
    comprobar("cancelar un pagado", d(pagado.clone().cancelar()), "err: pagado (30 €)");
    comprobar("pagar dos veces", d(pagado.pagar(10)), "err: pagado (30 €)");
    comprobar("cancelar un pendiente", d(Pedido::Pendiente.cancelar()), "ok: cancelado");
    comprobar("pagar un cancelado", d(Pedido::Cancelado.pagar(5)), "err: cancelado");
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
