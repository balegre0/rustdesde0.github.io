use std::num::ParseIntError;

fn doble(texto: &str) -> Result<i32, ParseIntError> {
    let n: i32 = texto.trim().parse()?; // si falla, devuelve el error
    Ok(n * 2)
}

fn main() {
    // 1. match con los dos casos
    for t in ["21", "hola"] {
        match doble(t) {
            Ok(v) => println!("{t} -> {v}"),
            Err(e) => println!("{t} -> error: {e}"),
        }
    }
    // 2. métodos
    println!("{}", doble("x").unwrap_or(-1));
    println!("{:?}", doble("5").map(|v| v + 1));
    println!("{:?}", doble("x").map_err(|e| e.to_string()));
    println!("{:?}", doble("4").ok());
    println!("{}", doble("4").is_ok());
    // 3. and_then encadena operaciones que pueden fallar
    let r = doble("3").and_then(|v| if v > 5 { Ok(v) } else { "no".parse::<i32>() });
    println!("{r:?}");
}
