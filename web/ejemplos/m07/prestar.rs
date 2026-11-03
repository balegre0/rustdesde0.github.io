fn mostrar(s: &String) {
    println!("  mostrar recibe un préstamo de: {s}");
}

fn main() {
    let saludo = String::from("hola");
    println!("1. saludo es la dueña: {saludo}");
    mostrar(&saludo);
    mostrar(&saludo);
    println!("2. saludo sigue siendo la dueña: {saludo}");
    let r = &saludo;
    println!("3. r es un préstamo: {r} (mide {} bytes)", r.len());
}
