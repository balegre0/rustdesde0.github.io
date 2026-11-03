use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // 1. Lanzar otro programa: en el navegador no hay procesos, así que falla
    match Command::new("rustc").arg("--version").output() {
        Ok(salida) => println!("rustc dice: {}", String::from_utf8_lossy(&salida.stdout)),
        Err(e) => println!("no se pudo lanzar rustc: {e}"),
    }
    // 2. El código de salida le dice al sistema si todo fue bien (0) o no (otro valor)
    println!("termino con código 2");
    ExitCode::from(2)
}
