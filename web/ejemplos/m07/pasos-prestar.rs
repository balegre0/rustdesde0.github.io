fn longitud(texto: &String) -> usize {
    texto.len()
}

fn main() {
    let nombre = String::from("Lía");
    let n = longitud(&nombre);
    println!("{nombre} tiene {n} bytes");
}
