
fn main() {
    let mut fallos = 0;
    {
        let obtenido = rectangulo(4, 3);
        let esperado = (12, 14);
        if obtenido == esperado {
            println!("OK: rectangulo(4, 3) = {:?}", obtenido);
        } else {
            println!("FALLO: rectangulo(4, 3) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = rectangulo(5, 5);
        let esperado = (25, 20);
        if obtenido == esperado {
            println!("OK: rectangulo(5, 5) = {:?}", obtenido);
        } else {
            println!("FALLO: rectangulo(5, 5) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = rectangulo(0, 7);
        let esperado = (0, 14);
        if obtenido == esperado {
            println!("OK: rectangulo(0, 7) = {:?}", obtenido);
        } else {
            println!("FALLO: rectangulo(0, 7) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = rectangulo(1, 1);
        let esperado = (1, 4);
        if obtenido == esperado {
            println!("OK: rectangulo(1, 1) = {:?}", obtenido);
        } else {
            println!("FALLO: rectangulo(1, 1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = rectangulo(100, 2);
        let esperado = (200, 204);
        if obtenido == esperado {
            println!("OK: rectangulo(100, 2) = {:?}", obtenido);
        } else {
            println!("FALLO: rectangulo(100, 2) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
