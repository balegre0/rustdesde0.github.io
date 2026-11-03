fn saludar(nombre: String) {
    // 2. Aquí dentro `nombre` es la dueña
    println!("¡Hola, {nombre}!");
} // 3. `nombre` sale de ámbito: se libera el texto

fn main() {
    let n = String::from("Ana");
    // 1. Pasar `n` es igual que escribir `let nombre = n;`: se mueve
    saludar(n);
    println!("main sigue, pero ya no es dueño de ningún texto");
}
