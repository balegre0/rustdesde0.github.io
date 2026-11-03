fn nombre_completo(nombre: &str, apellido: &str) -> &str {
    let completo = format!("{nombre} {apellido}");
    &completo
}

fn main() {
    println!("{}", nombre_completo("Ana", "Ruiz"));
}
