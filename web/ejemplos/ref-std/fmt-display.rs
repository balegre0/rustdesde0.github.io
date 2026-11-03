use std::fmt;

struct Temperatura {
    grados: f64,
}

impl fmt::Display for Temperatura {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // respeta la precisión que pida quien imprime, o 1 decimal
        let p = f.precision().unwrap_or(1);
        write!(f, "{:.*} °C", p, self.grados)
    }
}

fn main() {
    let t = Temperatura { grados: 21.456 };
    println!("{t}");
    println!("{t:.2}");
    // Display da también to_string()
    let s: String = t.to_string();
    println!("{} ({} bytes)", s, s.len());
}
