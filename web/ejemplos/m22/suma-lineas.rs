use std::io::{self, BufRead};

fn main() {
    // 1. Bloqueamos stdin una vez y leemos línea a línea con BufRead
    let stdin = io::stdin();
    let mut total = 0i64;
    let mut malas = 0;
    for (n, linea) in stdin.lock().lines().enumerate() {
        // 2. Cada línea es un io::Result<String>: leer puede fallar
        let linea = linea.expect("no se pudo leer stdin");
        // 3. Convertir el texto en número; si no se puede, lo contamos
        match linea.trim().parse::<i64>() {
            Ok(x) => total += x,
            Err(_) => {
                malas += 1;
                eprintln!("línea {}: {:?} no es un número", n + 1, linea);
            }
        }
    }
    println!("total = {total} ({malas} línea(s) ignorada(s))");
}
