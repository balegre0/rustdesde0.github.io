fn main() {
    let x = Cuenta { id: 1, saldo: Mutex::new(100) };
    let y = Cuenta { id: 2, saldo: Mutex::new(50) };
    let saldos = || (*x.saldo.lock().unwrap(), *y.saldo.lock().unwrap());
    let mut fallos = 0;
    let mut comprobar = |que: &str, ok: bool| {
        if ok {
            println!("OK: {que}");
        } else {
            println!("FALLO: {que}");
            fallos += 1;
        }
    };
    comprobar("x -> y 30", transferir(&x, &y, 30).is_ok() && saldos() == (70, 80));
    comprobar("y -> x 80 (en el otro sentido)", transferir(&y, &x, 80).is_ok() && saldos() == (150, 0));
    comprobar("y -> x 1 sin saldo", transferir(&y, &x, 1) == Err(String::from("saldo insuficiente")) && saldos() == (150, 0));
    comprobar("a la misma cuenta", transferir(&x, &x, 10) == Err(String::from("misma cuenta")));
    comprobar("los candados quedan libres", x.saldo.try_lock().is_ok() && y.saldo.try_lock().is_ok());
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
