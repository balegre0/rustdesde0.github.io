use std::io::{self, Write};

fn main() {
    let mut nombre = String::new();
    print!("¿Cómo te llamas? ");
    io::stdout().flush().unwrap(); // que la pregunta salga antes de leer
    io::stdin().read_line(&mut nombre).unwrap();
    let nombre = nombre.trim(); // quita el salto de línea

    print!("¿Y cuántos años tienes? ");
    io::stdout().flush().unwrap();
    let mut linea = String::new();
    let leidos = io::stdin().read_line(&mut linea).unwrap();
    match linea.trim().parse::<u32>() {
        Ok(edad) => println!("\nHola, {nombre}. El año que viene tendrás {}.", edad + 1),
        Err(e) => println!("\nEso no es un número: {e}"),
    }
    println!("(se leyeron {leidos} bytes en la segunda línea)");
}
