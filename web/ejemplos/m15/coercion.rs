fn saluda(nombre: &str) {
    println!("¡Hola, {nombre}!");
}

fn main() {
    // 1. Un &str directo.
    saluda("Ana");
    // 2. &String se convierte solo en &str (String: Deref<Target = str>).
    let s = String::from("Íñigo");
    saluda(&s);
    // 3. &Box<String> → &String → &str: dos pasos automáticos.
    let b = Box::new(String::from("Begoña"));
    saluda(&b);
}
