use std::io::{self, Write};

fn main() {
    let mut salida = io::stdout().lock();
    // Con el trait en ámbito, sus métodos están disponibles
    writeln!(salida, "hola").unwrap();
}
