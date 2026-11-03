use std::io::{BufRead, Cursor};

// Reto: devuelve (líneas, palabras) leyendo de cualquier cosa que implemente BufRead.
// Pista: .lines() y split_whitespace().
fn contar(entrada: impl BufRead) -> (usize, usize) {
    let mut lineas = 0;
    let mut palabras = 0;
    for linea in entrada.lines() {
        let linea = linea.expect("lectura en memoria: no falla");
        lineas += 1;
        palabras += linea.split_whitespace().count();
    }
    (lineas, palabras)
}

fn main() {
    // Cursor convierte un texto en memoria en algo que implementa BufRead, igual que stdin
    let texto = "hola mundo\n\nRust  desde   cero\n";
    let (l, p) = contar(Cursor::new(texto));
    println!("{l} líneas, {p} palabras");
}
