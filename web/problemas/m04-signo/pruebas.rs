
fn main() {
    let mut fallos = 0;
    {
        let obtenido = signo(42);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: signo(42) = {:?}", obtenido);
        } else {
            println!("FALLO: signo(42) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = signo(-7);
        let esperado = -1;
        if obtenido == esperado {
            println!("OK: signo(-7) = {:?}", obtenido);
        } else {
            println!("FALLO: signo(-7) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = signo(0);
        let esperado = 0;
        if obtenido == esperado {
            println!("OK: signo(0) = {:?}", obtenido);
        } else {
            println!("FALLO: signo(0) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = signo(1);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: signo(1) = {:?}", obtenido);
        } else {
            println!("FALLO: signo(1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = signo(i32::MIN);
        let esperado = -1;
        if obtenido == esperado {
            println!("OK: signo(i32::MIN) = {:?}", obtenido);
        } else {
            println!("FALLO: signo(i32::MIN) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = signo(i32::MAX);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: signo(i32::MAX) = {:?}", obtenido);
        } else {
            println!("FALLO: signo(i32::MAX) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
