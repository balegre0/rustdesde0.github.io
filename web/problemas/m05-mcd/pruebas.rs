
fn main() {
    let mut fallos = 0;
    {
        let obtenido = mcd(48, 36);
        let esperado = 12;
        if obtenido == esperado {
            println!("OK: mcd(48, 36) = {:?}", obtenido);
        } else {
            println!("FALLO: mcd(48, 36) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = mcd(36, 48);
        let esperado = 12;
        if obtenido == esperado {
            println!("OK: mcd(36, 48) = {:?}", obtenido);
        } else {
            println!("FALLO: mcd(36, 48) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = mcd(17, 5);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: mcd(17, 5) = {:?}", obtenido);
        } else {
            println!("FALLO: mcd(17, 5) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = mcd(0, 9);
        let esperado = 9;
        if obtenido == esperado {
            println!("OK: mcd(0, 9) = {:?}", obtenido);
        } else {
            println!("FALLO: mcd(0, 9) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = mcd(9, 0);
        let esperado = 9;
        if obtenido == esperado {
            println!("OK: mcd(9, 0) = {:?}", obtenido);
        } else {
            println!("FALLO: mcd(9, 0) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = mcd(1071, 462);
        let esperado = 21;
        if obtenido == esperado {
            println!("OK: mcd(1071, 462) = {:?}", obtenido);
        } else {
            println!("FALLO: mcd(1071, 462) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = mcd(1 << 40, 1 << 20);
        let esperado = 1 << 20;
        if obtenido == esperado {
            println!("OK: mcd(1 << 40, 1 << 20) = {:?}", obtenido);
        } else {
            println!("FALLO: mcd(1 << 40, 1 << 20) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
