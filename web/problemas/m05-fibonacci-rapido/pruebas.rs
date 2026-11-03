
fn main() {
    let mut fallos = 0;
    {
        let obtenido = fibonacci(0);
        let esperado = 0;
        if obtenido == esperado {
            println!("OK: fibonacci(0) = {:?}", obtenido);
        } else {
            println!("FALLO: fibonacci(0) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fibonacci(1);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: fibonacci(1) = {:?}", obtenido);
        } else {
            println!("FALLO: fibonacci(1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fibonacci(2);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: fibonacci(2) = {:?}", obtenido);
        } else {
            println!("FALLO: fibonacci(2) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fibonacci(10);
        let esperado = 55;
        if obtenido == esperado {
            println!("OK: fibonacci(10) = {:?}", obtenido);
        } else {
            println!("FALLO: fibonacci(10) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fibonacci(50);
        let esperado = 12586269025;
        if obtenido == esperado {
            println!("OK: fibonacci(50) = {:?}", obtenido);
        } else {
            println!("FALLO: fibonacci(50) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fibonacci(90);
        let esperado = 2880067194370816120;
        if obtenido == esperado {
            println!("OK: fibonacci(90) = {:?}", obtenido);
        } else {
            println!("FALLO: fibonacci(90) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
