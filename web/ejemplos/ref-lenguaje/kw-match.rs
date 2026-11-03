enum Moneda { Centimo, Euro(u32), Billete { valor: u32 } }

fn valor(m: &Moneda) -> u32 {
    // 1. Una rama por variante; el compilador exige cubrirlas todas.
    match m {
        Moneda::Centimo => 1,
        Moneda::Euro(n) => n * 100,
        Moneda::Billete { valor } => valor * 100,
    }
}

fn main() {
    println!("{}", valor(&Moneda::Euro(2)));
    println!("{}", valor(&Moneda::Billete { valor: 5 }));
    println!("{}", valor(&Moneda::Centimo));
    // 2. Rangos, alternativas, guardas y `@`.
    for n in [0, 3, 7, 15, 42] {
        let d = match n {
            0 => "cero".to_string(),
            1 | 2 | 3 => "pequeño".to_string(),
            x @ 4..=9 if x % 2 == 1 => format!("{x}: impar de un dígito"),
            4..=9 => "par de un dígito".to_string(),
            _ => "grande".to_string(),
        };
        println!("{n} -> {d}");
    }
    // 3. `match` es una expresión: el valor de la rama elegida.
    let par = (1, -1);
    let cuadrante = match par {
        (0, _) | (_, 0) => "eje",
        (x, y) if x > 0 && y > 0 => "primero",
        (_, y) if y < 0 => "abajo",
        _ => "otro",
    };
    println!("{cuadrante}");
}
