fn main() {
    let o = Some(String::from("hola"));
    // En la edición 2024, `ref` explícito no se permite cuando
    // el patrón ya enlaza por referencia (al hacer match sobre `&o`).
    if let Some(ref s) = &o {
        println!("{s}");
    }
}
