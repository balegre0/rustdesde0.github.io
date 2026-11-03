fn main() {
    let puntos = 7;
    // Con `else`, el `if` tiene valor en todos los casos
    let premio = if puntos > 5 { 10 } else { 0 };
    println!("{premio}");
}
