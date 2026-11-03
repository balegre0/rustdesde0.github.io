use std::io::{self, Write};

fn main() {
    // 1. Preguntar y vaciar la salida antes de leer
    print!("¿Cómo te llamas? ");
    io::stdout().flush().unwrap();
    let mut nombre = String::new();
    io::stdin().read_line(&mut nombre).unwrap();

    // 2. Leer la edad y convertirla en número
    print!("¿Cuántos años tienes? ");
    io::stdout().flush().unwrap();
    let mut texto = String::new();
    io::stdin().read_line(&mut texto).unwrap();
    let edad: u32 = texto.trim().parse().unwrap_or(0);

    // 3. Responder
    println!("Hola, {}. Dentro de 10 años tendrás {}.", nombre.trim(), edad + 10);
}
