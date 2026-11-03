fn longitud(s: String) -> usize {
    s.len()
}

fn main() {
    let nombre = String::from("Lucía");
    let n = longitud(nombre);
    println!("{nombre} tiene {n} bytes");
}
