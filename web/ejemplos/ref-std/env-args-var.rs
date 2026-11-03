use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{} argumento(s); el primero suele ser el programa", args.len());
    match env::var("IDIOMA") {
        Ok(v) => println!("IDIOMA = {v}"),
        Err(env::VarError::NotPresent) => println!("IDIOMA no está definida"),
        Err(env::VarError::NotUnicode(_)) => println!("IDIOMA no es UTF-8"),
    }
    let idioma = env::var("IDIOMA").unwrap_or_else(|_| String::from("es"));
    println!("uso el idioma «{idioma}»");
}
