fn etiqueta(nombre: &str, edad: u32) -> String {
    format!("{nombre} ({edad})")
}

fn main() {
    let propio = String::from("Lucía");
    println!("{}", etiqueta("Bruno", 8));
    println!("{}", etiqueta(&propio, 31));
}
