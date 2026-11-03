
fn main() {
    let mut fallos = 0;
    {
        let obtenido = maximo3(4, 11, 6);
        let esperado = 11;
        if obtenido == esperado {
            println!("OK: maximo3(4, 11, 6) = {:?}", obtenido);
        } else {
            println!("FALLO: maximo3(4, 11, 6) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = maximo3(-2, -9, -5);
        let esperado = -2;
        if obtenido == esperado {
            println!("OK: maximo3(-2, -9, -5) = {:?}", obtenido);
        } else {
            println!("FALLO: maximo3(-2, -9, -5) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = maximo3(7, 7, 7);
        let esperado = 7;
        if obtenido == esperado {
            println!("OK: maximo3(7, 7, 7) = {:?}", obtenido);
        } else {
            println!("FALLO: maximo3(7, 7, 7) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = maximo3(1, 2, 3);
        let esperado = 3;
        if obtenido == esperado {
            println!("OK: maximo3(1, 2, 3) = {:?}", obtenido);
        } else {
            println!("FALLO: maximo3(1, 2, 3) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = maximo3(3, 2, 1);
        let esperado = 3;
        if obtenido == esperado {
            println!("OK: maximo3(3, 2, 1) = {:?}", obtenido);
        } else {
            println!("FALLO: maximo3(3, 2, 1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = maximo3(i32::MIN, 0, i32::MAX);
        let esperado = i32::MAX;
        if obtenido == esperado {
            println!("OK: maximo3(i32::MIN, 0, i32::MAX) = {:?}", obtenido);
        } else {
            println!("FALLO: maximo3(i32::MIN, 0, i32::MAX) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
