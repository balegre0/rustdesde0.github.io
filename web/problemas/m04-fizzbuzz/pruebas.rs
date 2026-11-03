
fn main() {
    let mut fallos = 0;
    {
        let obtenido = fizzbuzz(1);
        let esperado = String::from("1");
        if obtenido == esperado {
            println!("OK: fizzbuzz(1) = {:?}", obtenido);
        } else {
            println!("FALLO: fizzbuzz(1) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fizzbuzz(9);
        let esperado = String::from("Fizz");
        if obtenido == esperado {
            println!("OK: fizzbuzz(9) = {:?}", obtenido);
        } else {
            println!("FALLO: fizzbuzz(9) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fizzbuzz(10);
        let esperado = String::from("Buzz");
        if obtenido == esperado {
            println!("OK: fizzbuzz(10) = {:?}", obtenido);
        } else {
            println!("FALLO: fizzbuzz(10) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fizzbuzz(15);
        let esperado = String::from("FizzBuzz");
        if obtenido == esperado {
            println!("OK: fizzbuzz(15) = {:?}", obtenido);
        } else {
            println!("FALLO: fizzbuzz(15) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fizzbuzz(30);
        let esperado = String::from("FizzBuzz");
        if obtenido == esperado {
            println!("OK: fizzbuzz(30) = {:?}", obtenido);
        } else {
            println!("FALLO: fizzbuzz(30) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    {
        let obtenido = fizzbuzz(98);
        let esperado = String::from("98");
        if obtenido == esperado {
            println!("OK: fizzbuzz(98) = {:?}", obtenido);
        } else {
            println!("FALLO: fizzbuzz(98) = {:?}, se esperaba {:?}", obtenido, esperado);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    } else {
        println!("{fallos} caso(s) fallido(s)");
    }
}
