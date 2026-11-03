use std::io::{BufRead, Cursor};

fn buscar(consulta: &str, entrada: impl BufRead, ignorar_mayusculas: bool) -> Vec<(usize, String)> {
    let _ = Cursor::new("");
    let aguja = if ignorar_mayusculas { consulta.to_lowercase() } else { consulta.to_string() };
    let mut res = Vec::new();
    for (i, linea) in entrada.lines().enumerate() {
        let Ok(linea) = linea else { continue };
        let pajar = if ignorar_mayusculas { linea.to_lowercase() } else { linea.clone() };
        if pajar.contains(&aguja) {
            res.push((i + 1, linea));
        }
    }
    res
}
