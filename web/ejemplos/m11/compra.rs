fn main() {
    let mut lista: Vec<&str> = Vec::new();
    lista.push("pan");
    lista.push("leche");
    lista.push("huevos");
    lista.remove(1);
    println!("{:?}", lista);
}
