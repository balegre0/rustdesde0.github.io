fn main() {
    // 1. Buscamos dos números del 1 al 9 cuyo producto sea 42
    let mut encontrado = (0, 0);
    'filas: for a in 1..10 {
        for b in 1..10 {
            if a * b == 42 {
                encontrado = (a, b);
                // 2. `break 'filas` sale de LOS DOS bucles, no solo del interior
                break 'filas;
            }
        }
    }
    println!("primera pareja: {encontrado:?}");

    // 3. `continue 'filas` salta a la siguiente vuelta del bucle exterior
    'filas2: for a in 1..4 {
        for b in 1..4 {
            if b > a {
                continue 'filas2;
            }
            print!("({a},{b}) ");
        }
    }
    println!();

    // 4. Un bloque etiquetado también puede devolver un valor con `break`
    let n = 15;
    let tipo = 'clasificar: {
        if n % 15 == 0 {
            break 'clasificar "múltiplo de 15";
        }
        if n % 3 == 0 {
            break 'clasificar "múltiplo de 3";
        }
        "otro"
    };
    println!("{n}: {tipo}");
}
