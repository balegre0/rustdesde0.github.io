
fn main() {
    let mut fallos = 0;
    {
        let obtenido = factorial(0);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: factorial(0) = {:?}", obtenido);
        } else {
            println!("FALLO: factorial(0) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = factorial(1);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: factorial(1) = {:?}", obtenido);
        } else {
            println!("FALLO: factorial(1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = factorial(5);
        let esperado = 120;
        if obtenido == esperado {
            println!("OK: factorial(5) = {:?}", obtenido);
        } else {
            println!("FALLO: factorial(5) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = factorial(10);
        let esperado = 3628800;
        if obtenido == esperado {
            println!("OK: factorial(10) = {:?}", obtenido);
        } else {
            println!("FALLO: factorial(10) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = factorial(20);
        let esperado = 2432902008176640000;
        if obtenido == esperado {
            println!("OK: factorial(20) = {:?}", obtenido);
        } else {
            println!("FALLO: factorial(20) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
