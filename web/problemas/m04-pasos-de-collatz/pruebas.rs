
fn main() {
    let mut fallos = 0;
    {
        let obtenido = pasos_collatz(1);
        let esperado = 0;
        if obtenido == esperado {
            println!("OK: pasos_collatz(1) = {:?}", obtenido);
        } else {
            println!("FALLO: pasos_collatz(1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = pasos_collatz(2);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: pasos_collatz(2) = {:?}", obtenido);
        } else {
            println!("FALLO: pasos_collatz(2) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = pasos_collatz(6);
        let esperado = 8;
        if obtenido == esperado {
            println!("OK: pasos_collatz(6) = {:?}", obtenido);
        } else {
            println!("FALLO: pasos_collatz(6) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = pasos_collatz(27);
        let esperado = 111;
        if obtenido == esperado {
            println!("OK: pasos_collatz(27) = {:?}", obtenido);
        } else {
            println!("FALLO: pasos_collatz(27) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = pasos_collatz(97);
        let esperado = 118;
        if obtenido == esperado {
            println!("OK: pasos_collatz(97) = {:?}", obtenido);
        } else {
            println!("FALLO: pasos_collatz(97) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = pasos_collatz(871);
        let esperado = 178;
        if obtenido == esperado {
            println!("OK: pasos_collatz(871) = {:?}", obtenido);
        } else {
            println!("FALLO: pasos_collatz(871) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
