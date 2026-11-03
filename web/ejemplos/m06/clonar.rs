fn main() {
    let original = String::from("hola");
    // 1. `clone` pide memoria nueva en el heap y copia los bytes
    let mut copia = original.clone();
    copia.push_str(", mundo");

    // 2. Dos dueños, dos textos distintos en el heap
    println!("original = {original}");
    println!("copia    = {copia}");
    println!("¿mismos bytes? {}", original.as_ptr() == copia.as_ptr());
}
