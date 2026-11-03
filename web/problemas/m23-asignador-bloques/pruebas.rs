fn main() {
    let mut b = Bloques::new();
    let mut fallos = 0;
    let mut comprobar = |que: &str, ok: bool| {
        if ok {
            println!("OK: {que}");
        } else {
            println!("FALLO: {que}");
            fallos += 1;
        }
    };
    comprobar("8 libres al empezar", b.libres() == 8);
    comprobar("reserva el 0, el 1 y el 2", b.reservar() == Some(0) && b.reservar() == Some(1) && b.reservar() == Some(2));
    comprobar("liberar el 1", b.liberar(1) == Ok(()));
    comprobar("el siguiente reutiliza el 1", b.reservar() == Some(1));
    comprobar("doble liberación", b.liberar(5) == Err("doble liberación"));
    comprobar("fuera de rango", b.liberar(8) == Err("fuera de rango"));
    for _ in 0..5 {
        b.reservar();
    }
    comprobar("lleno: None", b.reservar().is_none() && b.libres() == 0);
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
