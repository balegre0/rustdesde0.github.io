fn main() {
    let nombre = String::from("Ana");
    let mut lista = Vec::new();
    for _ in 0..2 {
        lista.push(nombre);
    }
    println!("{}", lista.len());
}
