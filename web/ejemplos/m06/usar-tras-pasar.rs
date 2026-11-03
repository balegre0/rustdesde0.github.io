fn saludar(nombre: String) {
    println!("¡Hola, {nombre}!");
}

fn main() {
    let n = String::from("Ana");
    saludar(n);
    println!("Adiós, {n}");
}
