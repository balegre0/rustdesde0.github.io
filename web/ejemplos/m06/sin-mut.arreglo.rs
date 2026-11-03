fn main() {
    // `mut` le da a la dueña el permiso W (escribir)
    let mut nota = String::from("comprar");
    nota.push_str(" pan");
    println!("{nota}");
}
