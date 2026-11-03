fn guardar(nombre: &str) -> Result<(), String> {
    if nombre.is_empty() {
        Err(String::from("nombre vacío"))
    } else {
        Ok(())
    }
}

fn main() {
    guardar("");
    println!("guardado (¿seguro?)");
}
