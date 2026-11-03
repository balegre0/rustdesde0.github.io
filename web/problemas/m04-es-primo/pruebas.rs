
fn main() {
    let mut fallos = 0;
    {
        let obtenido = es_primo(0);
        let esperado = false;
        if obtenido == esperado {
            println!("OK: es_primo(0) = {:?}", obtenido);
        } else {
            println!("FALLO: es_primo(0) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = es_primo(1);
        let esperado = false;
        if obtenido == esperado {
            println!("OK: es_primo(1) = {:?}", obtenido);
        } else {
            println!("FALLO: es_primo(1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = es_primo(2);
        let esperado = true;
        if obtenido == esperado {
            println!("OK: es_primo(2) = {:?}", obtenido);
        } else {
            println!("FALLO: es_primo(2) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = es_primo(9);
        let esperado = false;
        if obtenido == esperado {
            println!("OK: es_primo(9) = {:?}", obtenido);
        } else {
            println!("FALLO: es_primo(9) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = es_primo(91);
        let esperado = false;
        if obtenido == esperado {
            println!("OK: es_primo(91) = {:?}", obtenido);
        } else {
            println!("FALLO: es_primo(91) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = es_primo(97);
        let esperado = true;
        if obtenido == esperado {
            println!("OK: es_primo(97) = {:?}", obtenido);
        } else {
            println!("FALLO: es_primo(97) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = es_primo(7919);
        let esperado = true;
        if obtenido == esperado {
            println!("OK: es_primo(7919) = {:?}", obtenido);
        } else {
            println!("FALLO: es_primo(7919) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = es_primo(4294967291);
        let esperado = true;
        if obtenido == esperado {
            println!("OK: es_primo(4294967291) = {:?}", obtenido);
        } else {
            println!("FALLO: es_primo(4294967291) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = es_primo(4294967295);
        let esperado = false;
        if obtenido == esperado {
            println!("OK: es_primo(4294967295) = {:?}", obtenido);
        } else {
            println!("FALLO: es_primo(4294967295) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
