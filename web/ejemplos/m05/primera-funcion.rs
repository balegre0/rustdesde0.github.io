fn main() {
    println!("empieza main");
    // 1. Llamar a una función: su nombre y paréntesis
    saludar();
    saludar();
    println!("termina main");
}

// 2. Definir una función: `fn`, nombre, paréntesis y un bloque.
//    Puede estar antes o después de `main`: el orden no importa.
fn saludar() {
    println!("  ¡hola desde saludar!");
}
