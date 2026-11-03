fn saludar(nombre: &str, veces: u32) {
    for _ in 0..veces {
        println!("hola, {nombre}");
    }
}

fn main() {
    saludar("Ana");
}
