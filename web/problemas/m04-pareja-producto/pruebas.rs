
fn main() {
    let mut fallos = 0;
    {
        let obtenido = primera_pareja(12);
        let esperado = (2, 6);
        if obtenido == esperado {
            println!("OK: primera_pareja(12) = {:?}", obtenido);
        } else {
            println!("FALLO: primera_pareja(12) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = primera_pareja(81);
        let esperado = (9, 9);
        if obtenido == esperado {
            println!("OK: primera_pareja(81) = {:?}", obtenido);
        } else {
            println!("FALLO: primera_pareja(81) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = primera_pareja(11);
        let esperado = (0, 0);
        if obtenido == esperado {
            println!("OK: primera_pareja(11) = {:?}", obtenido);
        } else {
            println!("FALLO: primera_pareja(11) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = primera_pareja(1);
        let esperado = (1, 1);
        if obtenido == esperado {
            println!("OK: primera_pareja(1) = {:?}", obtenido);
        } else {
            println!("FALLO: primera_pareja(1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = primera_pareja(36);
        let esperado = (4, 9);
        if obtenido == esperado {
            println!("OK: primera_pareja(36) = {:?}", obtenido);
        } else {
            println!("FALLO: primera_pareja(36) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = primera_pareja(0);
        let esperado = (0, 0);
        if obtenido == esperado {
            println!("OK: primera_pareja(0) = {:?}", obtenido);
        } else {
            println!("FALLO: primera_pareja(0) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = primera_pareja(24);
        let esperado = (3, 8);
        if obtenido == esperado {
            println!("OK: primera_pareja(24) = {:?}", obtenido);
        } else {
            println!("FALLO: primera_pareja(24) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
