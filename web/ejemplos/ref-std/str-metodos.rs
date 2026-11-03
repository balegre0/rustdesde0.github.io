fn main() {
    let frase = "  Rust es rápido y seguro  ";
    // 1. limpiar y comparar
    let t = frase.trim();
    println!("[{t}]");
    println!("{}", t.starts_with("Rust"));
    println!("{}", t.contains("seguro"));
    println!("{:?}", t.find("es"));
    // 2. dividir
    let palabras: Vec<&str> = t.split(' ').collect();
    println!("{palabras:?}");
    println!("{:?}", t.split_whitespace().count());
    println!("{:?}", "clave=valor".split_once('='));
    // 3. transformar (crean un String nuevo)
    println!("{}", t.to_uppercase());
    println!("{}", t.replace("rápido", "veloz"));
    // 4. caracteres y bytes
    println!("{:?}", "año".chars().rev().collect::<String>());
    println!("{:?}", "año".char_indices().collect::<Vec<_>>());
    // 5. convertir a número
    println!("{:?}", "42".parse::<u8>());
    println!("{}", "línea 1\nlínea 2".lines().count());
}
