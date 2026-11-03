fn guardar(nota: String) {
    println!("guardo: {nota}");
}

fn main() {
    let nota = String::from("comprar pan");
    for _ in 0..3 {
        // Cada vuelta entrega una copia nueva, con su propio dueño
        guardar(nota.clone());
    }
} // aquí se libera la `nota` original
