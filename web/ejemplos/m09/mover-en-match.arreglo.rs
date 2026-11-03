struct Carta {
    texto: String,
}

fn main() {
    let correo = Some(Carta { texto: String::from("hola") });
    match &correo {
        Some(c) => println!("{}", c.texto),
        None => {}
    }
    println!("¿había carta? {}", correo.is_some());
}
