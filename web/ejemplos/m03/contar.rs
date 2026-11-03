fn main() {
    let mut frase = String::from("El niño");
    frase.push_str(" come piña");
    println!("{frase}: {} bytes, {} caracteres", frase.len(), frase.chars().count());
}
