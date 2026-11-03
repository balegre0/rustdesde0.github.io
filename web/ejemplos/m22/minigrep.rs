// Jefe del mundo 22: una herramienta de terminal completa.
// Entrada: la primera línea es el patrón; el resto, el texto donde buscar.
// Salida: las líneas que contienen el patrón (sin distinguir mayúsculas), con su número,
// y un resumen tipo `wc` (líneas, palabras, bytes). Los avisos van por stderr.
use std::io::{self, BufRead, BufWriter, Write};
use std::process::ExitCode;

struct Resumen { lineas: usize, palabras: usize, bytes: usize, coincidencias: usize }

fn buscar(patron: &str, entrada: impl BufRead, salida: &mut impl Write) -> io::Result<Resumen> {
    let patron = patron.to_lowercase();
    let mut r = Resumen { lineas: 0, palabras: 0, bytes: 0, coincidencias: 0 };
    for (i, linea) in entrada.lines().enumerate() {
        let linea = linea?;
        r.lineas += 1;
        r.palabras += linea.split_whitespace().count();
        r.bytes += linea.len() + 1; // +1 por el salto de línea
        if linea.to_lowercase().contains(&patron) {
            r.coincidencias += 1;
            writeln!(salida, "{:>3}: {}", i + 1, linea)?;
        }
    }
    Ok(r)
}

fn main() -> ExitCode {
    let stdin = io::stdin();
    let mut entrada = stdin.lock();
    // 1. Leer el patrón
    let mut patron = String::new();
    if entrada.read_line(&mut patron).unwrap_or(0) == 0 || patron.trim().is_empty() {
        eprintln!("uso: la primera línea debe ser el patrón a buscar");
        return ExitCode::from(2);
    }
    let patron = patron.trim();

    // 2. Buscar escribiendo con buffer
    let mut salida = BufWriter::new(io::stdout().lock());
    let r = match buscar(patron, entrada, &mut salida) {
        Ok(r) => r,
        Err(e) => { eprintln!("error de lectura: {e}"); return ExitCode::FAILURE; }
    };
    let _ = writeln!(salida, "--- {:?}: {} coincidencia(s) | {} líneas, {} palabras, {} bytes",
        patron, r.coincidencias, r.lineas, r.palabras, r.bytes);
    let _ = salida.flush();

    // 3. Como grep: código 0 si hubo coincidencias, 1 si no
    if r.coincidencias > 0 { ExitCode::SUCCESS } else { ExitCode::from(1) }
}
