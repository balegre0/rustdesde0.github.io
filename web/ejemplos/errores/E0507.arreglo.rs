struct Caja {
    contenido: String,
}

fn main() {
    let c = Caja { contenido: String::from("libro") };
    let r = &c;
    let sacado = &r.contenido; // tomar prestado en vez de mover
    println!("{sacado}");
}
