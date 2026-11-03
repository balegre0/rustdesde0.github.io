enum Mensaje {
    Salir,
    Mover { x: i32, y: i32 },
    Escribir(String),
}

fn procesar(m: Mensaje) -> String {
    match m {
        Mensaje::Salir => String::from("adiós"),
        Mensaje::Mover { x, y } => format!("ir a {x},{y}"),
        Mensaje::Escribir(texto) => texto,
    }
}

fn main() {
    let lista = [Mensaje::Mover { x: 2, y: -1 }, Mensaje::Escribir(String::from("hola")), Mensaje::Salir];
    for m in lista {
        println!("{}", procesar(m));
    }
}
