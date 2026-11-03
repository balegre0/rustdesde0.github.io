
fn main() {
    let mut fallos = 0;
    {
        let obtenido = potencia(2, 10);
        let esperado = 1024;
        if obtenido == esperado {
            println!("OK: potencia(2, 10) = {:?}", obtenido);
        } else {
            println!("FALLO: potencia(2, 10) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = potencia(3, 13);
        let esperado = 1594323;
        if obtenido == esperado {
            println!("OK: potencia(3, 13) = {:?}", obtenido);
        } else {
            println!("FALLO: potencia(3, 13) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = potencia(7, 0);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: potencia(7, 0) = {:?}", obtenido);
        } else {
            println!("FALLO: potencia(7, 0) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = potencia(0, 5);
        let esperado = 0;
        if obtenido == esperado {
            println!("OK: potencia(0, 5) = {:?}", obtenido);
        } else {
            println!("FALLO: potencia(0, 5) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = potencia(1, 4000000000);
        let esperado = 1;
        if obtenido == esperado {
            println!("OK: potencia(1, 4000000000) = {:?}", obtenido);
        } else {
            println!("FALLO: potencia(1, 4000000000) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = potencia(2, 63);
        let esperado = 9223372036854775808;
        if obtenido == esperado {
            println!("OK: potencia(2, 63) = {:?}", obtenido);
        } else {
            println!("FALLO: potencia(2, 63) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = potencia(10, 19);
        let esperado = 10000000000000000000;
        if obtenido == esperado {
            println!("OK: potencia(10, 19) = {:?}", obtenido);
        } else {
            println!("FALLO: potencia(10, 19) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
