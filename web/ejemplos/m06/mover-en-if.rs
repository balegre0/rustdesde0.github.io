fn guardar(nota: String) {
    println!("guardo: {nota}");
}

fn main() {
    let nota = String::from("comprar pan");
    let urgente = true;
    if urgente {
        guardar(nota);
    }
    println!("{nota}");
}
