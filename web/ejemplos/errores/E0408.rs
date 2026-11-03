enum Mensaje {
    Texto(String),
    Numero(i32),
}

fn main() {
    let m = Mensaje::Numero(4);
    match m {
        Mensaje::Texto(t) | Mensaje::Numero(n) => println!("algo"),
    }
}
