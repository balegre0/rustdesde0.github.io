enum Forma {
    Rect(u32, u32),
}

fn main() {
    let f = Forma::Rect(2, 3);
    match f {
        Forma::Rect(ancho, alto) => println!("{ancho} x {alto}"),
    }
}
