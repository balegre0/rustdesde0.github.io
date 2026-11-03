fn json_simple(nombre: &str, edad: u32) -> String {
    format!("{{\"nombre\": {:?}, \"edad\": {}}}", nombre, edad)
}
