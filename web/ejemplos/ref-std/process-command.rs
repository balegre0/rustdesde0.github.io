use std::process::Command;

fn main() {
    // lanzar otro programa; en el navegador no se puede
    match Command::new("rustc").arg("--version").output() {
        Ok(salida) => println!("{}", String::from_utf8_lossy(&salida.stdout)),
        Err(e) => println!("no se pudo ejecutar: {e}"),
    }
}
