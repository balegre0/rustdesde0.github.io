enum Mensaje {
    Texto(String),
    Numero(i32),
}

fn main() {
    for m in [Mensaje::Numero(4), Mensaje::Texto(String::from("hola"))] {
        match m {
            Mensaje::Texto(t) => println!("texto: {t}"),
            Mensaje::Numero(n) => println!("número: {n}"),
        }
    }
}
