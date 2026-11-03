fn main() {
    // 1. Un texto de un millón de bytes en el heap
    let libro = "a".repeat(1_000_000);
    let bytes_antes = libro.as_ptr();

    // 2. Moverlo solo copia la ficha de la pila
    let otro = libro;

    // 3. El millón de bytes sigue donde estaba
    println!("bytes del texto: {}", otro.len());
    println!("tamaño de la ficha: {} bytes", size_of_val(&otro));
    println!("¿mismos bytes en el heap? {}", bytes_antes == otro.as_ptr());
}
