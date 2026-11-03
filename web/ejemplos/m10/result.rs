// 1. Result<T, E>: o Ok(valor) o Err(error). Como Option, pero el fallo trae información.
fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("división entre cero"));
    }
    Ok(a / b)
}

fn main() {
    // 2. Tratar los dos casos con match.
    match dividir(10, 3) {
        Ok(r) => println!("10 / 3 = {r}"),
        Err(e) => println!("fallo: {e}"),
    }
    match dividir(1, 0) {
        Ok(r) => println!("1 / 0 = {r}"),
        Err(e) => println!("fallo: {e}"),
    }

    // 3. parse devuelve Result: el texto puede no ser un número.
    let entrada = " 27\n"; // lo que llega al leer una línea del teclado
    match entrada.trim().parse::<u8>() {
        Ok(edad) => println!("edad: {edad}"),
        Err(e) => println!("no es una edad: {e}"),
    }

    // 4. El error de parse es un valor de verdad: tiene tipo y se puede inspeccionar.
    let fallo = "300".parse::<u8>().unwrap_err();
    println!("Debug del error: {:?}", fallo);
    println!("Display del error: {}", fallo);

    // 5. is_ok / is_err, y .ok() para convertirlo en Option (perdiendo el error).
    println!("{} {} {:?}", "8".parse::<i32>().is_ok(), "x".parse::<i32>().is_err(), "x".parse::<i32>().ok());
}
