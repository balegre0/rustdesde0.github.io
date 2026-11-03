fn main() {
    // 1. Salir de un bucle.
    let mut i = 0;
    while i < 100 {
        if i == 3 { break; }
        i += 1;
    }
    println!("i = {i}");
    // 2. `loop` puede devolver un valor con `break valor`.
    let mut n = 1;
    let primera = loop {
        n *= 2;
        if n > 50 { break n; }
    };
    println!("{primera}");
    // 3. Con etiqueta: salir de un bucle exterior.
    'fuera: for x in 0..5 {
        for y in 0..5 {
            if x * y == 6 {
                println!("encontrado: {x} * {y}");
                break 'fuera;
            }
        }
    }
    // 4. Salir de un bloque etiquetado (desde Rust 1.65).
    let signo = 'b: {
        if n < 0 { break 'b "negativo"; }
        "no negativo"
    };
    println!("{signo}");
}
