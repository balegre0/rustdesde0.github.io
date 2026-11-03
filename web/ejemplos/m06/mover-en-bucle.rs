fn guardar(nota: String) {
    println!("guardo: {nota}");
}

fn main() {
    let nota = String::from("comprar pan");
    for _ in 0..3 {
        guardar(nota);
    }
}
