fn main() {
    let frase = "  Hola, mundo  ";
    let limpia = frase.trim();
    let partes: Vec<&str> = limpia.split(", ").collect();
    println!("{:?} {:?}", limpia, partes);
}
