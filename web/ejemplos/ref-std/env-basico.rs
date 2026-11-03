use std::env;

fn main() {
    // 1. argumentos de la línea de órdenes (el primero es el programa)
    let args: Vec<String> = env::args().collect();
    println!("{} argumento(s): {:?}", args.len(), args);
    // 2. variables de entorno
    match env::var("IDIOMA") {
        Ok(v) => println!("IDIOMA = {v}"),
        Err(e) => println!("IDIOMA: {e}"),
    }
    println!("{}", env::vars().count());
    // 3. carpeta actual
    println!("{:?}", env::current_dir());
    // 4. constantes del sistema
    println!("{} {}", env::consts::ARCH, env::consts::OS);
}
