fn main() {
    let mut a = Anillo::new();
    let mut fallos = 0;
    let mut comprobar = |que: &str, ok: bool| {
        if ok {
            println!("OK: {que}");
        } else {
            println!("FALLO: {que}");
            fallos += 1;
        }
    };
    comprobar("vacío: None", a.sacar().is_none());
    comprobar("meter 4", (1..=4).all(|b| a.meter(b).is_ok()) && a.len() == 4);
    comprobar("lleno: Err(5)", a.meter(5) == Err(5));
    comprobar("saca el más antiguo", a.sacar() == Some(1) && a.sacar() == Some(2));
    comprobar("da la vuelta", a.meter(6).is_ok() && a.meter(7).is_ok() && a.len() == 4);
    let mut resto = Vec::new();
    while let Some(b) = a.sacar() {
        resto.push(b);
    }
    comprobar("orden FIFO tras dar la vuelta", resto == vec![3, 4, 6, 7]);
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
