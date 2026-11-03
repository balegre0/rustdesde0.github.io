fn nombre_completo(nombre: &str, apellido: &str) -> String {
    format!("{nombre} {apellido}")
}

fn main() {
    println!("{}", nombre_completo("Ana", "Ruiz"));
}
