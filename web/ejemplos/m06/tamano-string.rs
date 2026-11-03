fn main() {
    // 1. La ficha que vive en la pila: puntero + capacidad + longitud
    println!("ficha de un String: {} bytes", size_of::<String>());
    println!("cada campo (usize): {} bytes", size_of::<usize>());
    // 2. Lo que vive en el heap: los bytes del texto
    let s = String::from("año");
    println!("bytes de «{s}» en el heap: {}", s.len());
}
