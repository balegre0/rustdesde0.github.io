// 1. Buscar puede fallar: devolvemos Option<usize>, no un número «mágico».
fn posicion(nombres: &[&str], buscado: &str) -> Option<usize> {
    for (i, n) in nombres.iter().enumerate() {
        if *n == buscado {
            return Some(i);
        }
    }
    None
}

fn main() {
    let nombres = ["Ana", "Iñaki", "Lucía"];

    // 2. match: tratar los dos casos a mano.
    match posicion(&nombres, "Iñaki") {
        Some(i) => println!("Iñaki está en la posición {i}"),
        None => println!("no está"),
    }

    // 3. Métodos útiles de Option.
    let p = posicion(&nombres, "Pepe");
    println!("is_some = {}, is_none = {}", p.is_some(), p.is_none());
    println!("unwrap_or: {}", p.unwrap_or(999));

    // 4. map: transformar lo de dentro SI hay algo.
    let doble = posicion(&nombres, "Lucía").map(|i| i * 2);
    println!("map: {:?}", doble);

    // 5. Option también sirve para datos que pueden faltar.
    let segundo_apellido: Option<&str> = None;
    println!("segundo apellido: {}", segundo_apellido.unwrap_or("(ninguno)"));
}
