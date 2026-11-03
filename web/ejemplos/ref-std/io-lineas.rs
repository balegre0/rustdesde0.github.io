use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut total = 0;
    // lines() lee hasta el final de la entrada
    for (i, linea) in stdin.lock().lines().enumerate() {
        let linea = linea.unwrap();
        total += linea.len();
        println!("{}: {}", i + 1, linea);
    }
    println!("{total} letras");
}
