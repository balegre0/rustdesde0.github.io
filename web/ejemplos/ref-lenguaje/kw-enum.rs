// 1. Variantes sin datos, con datos tipo tupla y con campos con nombre.
#[derive(Debug)]
enum Mensaje {
    Salir,
    Mover { x: i32, y: i32 },
    Escribir(String),
}

// 2. Discriminantes explícitos.
#[derive(Debug, Clone, Copy)]
enum Http {
    Ok = 200,
    NoEncontrado = 404,
}

fn describir(m: &Mensaje) -> String {
    match m {
        Mensaje::Salir => "salir".to_string(),
        Mensaje::Mover { x, y } => format!("mover a ({x}, {y})"),
        Mensaje::Escribir(t) => format!("escribir «{t}»"),
    }
}

fn main() {
    let ms = [Mensaje::Salir, Mensaje::Mover { x: 1, y: -2 }, Mensaje::Escribir("hola".into())];
    for m in &ms {
        println!("{}", describir(m));
    }
    println!("{:?} = {}", Http::NoEncontrado, Http::NoEncontrado as i32);
    println!("{:?}", Http::Ok);
}
