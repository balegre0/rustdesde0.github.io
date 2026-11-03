fn main() {
    let s = String::from("secreto");
    // Primero lo usamos; después lo liberamos
    println!("{s}");
    drop(s);
}
