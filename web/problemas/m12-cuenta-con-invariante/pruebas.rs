fn main() {
    let mut fallos = 0;
    let mut comprobar = |nombre: &str, ok: bool| {
        if ok {
            println!("OK: {nombre}");
        } else {
            println!("FALLO: {nombre}");
            fallos += 1;
        }
    };
    let mut c = banco::Cuenta::nueva();
    comprobar("empieza a 0", c.saldo() == 0);
    c.ingresar(50);
    c.ingresar(25);
    comprobar("ingresar suma", c.saldo() == 75);
    comprobar("retirar 30 va bien", c.retirar(30) == Ok(()));
    comprobar("queda 45", c.saldo() == 45);
    comprobar("retirar 100 falla con el mensaje", c.retirar(100) == Err(String::from("saldo insuficiente: hay 45, pides 100")));
    comprobar("un retiro fallido no cambia nada", c.saldo() == 45);
    comprobar("retirar todo deja 0", c.retirar(45).is_ok() && c.saldo() == 0);
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
