use std::env;

fn main() {
    // 1. args() da los argumentos de la línea de órdenes; el primero es el nombre del programa
    let args: Vec<String> = env::args().skip(1).collect();
    println!("me pasaron {} argumento(s): {:?}", args.len(), args);

    // 2. Leer un argumento con un valor por defecto
    let nombre = args.first().map(String::as_str).unwrap_or("mundo");
    println!("hola, {nombre}");

    // 3. Variables de entorno: pueden no existir, así que devuelven Result
    match env::var("IDIOMA") {
        Ok(v) => println!("IDIOMA = {v}"),
        Err(e) => println!("IDIOMA no está definida ({e})"),
    }
    println!("hay {} variables de entorno", env::vars().count());
}
