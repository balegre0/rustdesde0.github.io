fn main() {
    let saludo = String::from("hola");
    let otro = saludo.clone(); // copia completa: cada variable tiene su String
    println!("{saludo}");
    println!("{otro}");
}
