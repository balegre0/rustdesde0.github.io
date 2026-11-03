// Haz que compile SIN usar .clone() y sin cambiar lo que se imprime.
fn anunciar(nombre: String) -> String {
    println!("Llega {nombre}");
    nombre
}

fn main() {
    let mut fila: Vec<String> = Vec::new();
    let a = String::from("Ana");
    let b = String::from("Bea");

    // 1. Recogemos la propiedad que nos devuelve `anunciar`
    let a = anunciar(a);
    fila.push(a);
    // 2. Usamos `b` antes de entregársela al Vec
    println!("{b} es la segunda");
    fila.push(b);

    // 3. Medimos antes de que el `for` consuma el Vec
    let total = fila.len();
    for persona in fila {
        println!("en la fila: {persona}");
    }
    println!("total: {total}");
}
