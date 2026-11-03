fn main() {
    let lenguaje = "Rust";
    let edicion = 2024;
    let mundos = vec![17, 18, 19, 20];
    println!("Aprendo {} (edición {})", lenguaje, edicion);
    println!("Mundos: {:?}", mundos);
    println!("Código: {}", stringify!(edicion + 1));
}
