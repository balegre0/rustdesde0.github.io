use std::collections::HashMap;
use std::fmt::Write as _; // importar un trait solo para usar sus métodos
use std::io::{self, Write};

mod geometria {
    pub mod formas {
        pub fn area_circulo(r: f64) -> f64 {
            std::f64::consts::PI * r * r
        }
    }
}

use geometria::formas::area_circulo as area; // renombrar con `as`

fn main() -> io::Result<()> {
    let mut notas: HashMap<&str, u32> = HashMap::new();
    notas.insert("Ana", 9);

    let mut texto = String::new();
    write!(texto, "Ana tiene un {}", notas["Ana"]).unwrap();

    let mut salida = io::stdout();
    writeln!(salida, "{texto}")?;
    writeln!(salida, "área: {:.3}", area(1.0))?;
    Ok(())
}
