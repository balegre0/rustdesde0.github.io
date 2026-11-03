fn main() {
    // 1. Sin `mut`: la dueña puede leer, pero no modificar
    let fija = String::from("hola");
    println!("{fija}");

    // 2. Con `mut`: la dueña puede además modificar
    let mut nota = String::from("comprar");
    nota.push_str(" pan");
    println!("{nota}");
}
