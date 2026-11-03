fn buscar(edades: &[(&str, u32)], quien: &str) -> Option<u32> {
    for (nombre, edad) in edades {
        if *nombre == quien {
            return Some(*edad);
        }
    }
    None
}

fn main() {
    let edades = [("Ana", 31), ("Luis", 25)];
    // 1. match: tratar los dos casos
    match buscar(&edades, "Ana") {
        Some(e) => println!("Ana tiene {e} años"),
        None => println!("no está"),
    }
    // 2. if let: solo nos interesa el caso Some
    if let Some(e) = buscar(&edades, "Luis") {
        println!("Luis tiene {e} años");
    }
    // 3. métodos: valor por defecto, transformar, encadenar
    println!("{}", buscar(&edades, "Eva").unwrap_or(0));
    println!("{:?}", buscar(&edades, "Ana").map(|e| e + 1));
    println!("{:?}", buscar(&edades, "Ana").filter(|e| *e > 40));
    println!("{}", buscar(&edades, "Eva").is_none());
}
