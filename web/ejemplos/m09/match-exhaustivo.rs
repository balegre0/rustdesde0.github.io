#[derive(Debug)]
enum Moneda {
    Cent1,
    Cent5,
    Cent10,
    Euro1,
}

// 1. match es una EXPRESIÓN: todos los brazos devuelven el mismo tipo.
fn valor_en_centimos(m: &Moneda) -> u32 {
    match m {
        Moneda::Cent1 => 1,
        Moneda::Cent5 => 5,
        Moneda::Cent10 => 10,
        // 2. Un brazo puede ser un bloque con varias sentencias.
        Moneda::Euro1 => {
            println!("  (¡un euro entero!)");
            100
        }
    }
}

fn main() {
    let bolsillo = [Moneda::Cent5, Moneda::Euro1, Moneda::Cent1];
    let mut total = 0;
    for m in &bolsillo {
        let v = valor_en_centimos(m);
        println!("{:?} vale {}", m, v);
        total += v;
    }
    println!("total: {total} céntimos");

    // 3. `_` (comodín) recoge todo lo que no hayas nombrado.
    for m in &bolsillo {
        let es_pequena = match m {
            Moneda::Euro1 => false,
            _ => true,
        };
        println!("{:?}: ¿pequeña? {}", m, es_pequena);
    }

    // 4. Con números, el comodín es casi obligatorio: hay demasiados valores.
    let dado: u8 = 4;
    match dado {
        1 => println!("uno"),
        6 => println!("seis"),
        otro => println!("salió un {otro}"),
    }
}
