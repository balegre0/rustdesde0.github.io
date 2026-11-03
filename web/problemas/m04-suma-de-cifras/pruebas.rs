
fn main() {
    let mut fallos = 0;
    {
        let obtenido = suma_cifras(0);
        let esperado = 0;
        if obtenido == esperado {
            println!("OK: suma_cifras(0) = {:?}", obtenido);
        } else {
            println!("FALLO: suma_cifras(0) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = suma_cifras(7);
        let esperado = 7;
        if obtenido == esperado {
            println!("OK: suma_cifras(7) = {:?}", obtenido);
        } else {
            println!("FALLO: suma_cifras(7) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = suma_cifras(4096);
        let esperado = 19;
        if obtenido == esperado {
            println!("OK: suma_cifras(4096) = {:?}", obtenido);
        } else {
            println!("FALLO: suma_cifras(4096) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = suma_cifras(999);
        let esperado = 27;
        if obtenido == esperado {
            println!("OK: suma_cifras(999) = {:?}", obtenido);
        } else {
            println!("FALLO: suma_cifras(999) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = suma_cifras(1000000);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: suma_cifras(1000000) = {:?}", obtenido);
        } else {
            println!("FALLO: suma_cifras(1000000) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = suma_cifras(u64::MAX);
        let esperado = 87;
        if obtenido == esperado {
            println!("OK: suma_cifras(u64::MAX) = {:?}", obtenido);
        } else {
            println!("FALLO: suma_cifras(u64::MAX) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
