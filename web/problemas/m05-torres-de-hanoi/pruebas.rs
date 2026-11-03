
fn main() {
    let mut fallos = 0;
    {
        let obtenido = movimientos_hanoi(0);
        let esperado = 0;
        if obtenido == esperado {
            println!("OK: movimientos_hanoi(0) = {:?}", obtenido);
        } else {
            println!("FALLO: movimientos_hanoi(0) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = movimientos_hanoi(1);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: movimientos_hanoi(1) = {:?}", obtenido);
        } else {
            println!("FALLO: movimientos_hanoi(1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = movimientos_hanoi(3);
        let esperado = 7;
        if obtenido == esperado {
            println!("OK: movimientos_hanoi(3) = {:?}", obtenido);
        } else {
            println!("FALLO: movimientos_hanoi(3) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = movimientos_hanoi(10);
        let esperado = 1023;
        if obtenido == esperado {
            println!("OK: movimientos_hanoi(10) = {:?}", obtenido);
        } else {
            println!("FALLO: movimientos_hanoi(10) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = movimientos_hanoi(64);
        let esperado = 18446744073709551615;
        if obtenido == esperado {
            println!("OK: movimientos_hanoi(64) = {:?}", obtenido);
        } else {
            println!("FALLO: movimientos_hanoi(64) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
