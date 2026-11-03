// Haz que compile SIN usar .clone() y sin cambiar lo que se imprime.
fn anunciar(nombre: String) -> String {
    println!("Llega {nombre}");
    nombre
}

fn main() {
    let mut fila: Vec<String> = Vec::new();
    let a = String::from("Ana");
    let b = String::from("Bea");

    anunciar(a);
    fila.push(a);
    fila.push(b);
    println!("{b} es la segunda");

    for persona in fila {
        println!("en la fila: {persona}");
    }
    println!("total: {}", fila.len());
}
