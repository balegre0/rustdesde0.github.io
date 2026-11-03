fn main() {
    let nombre = "Ana Martínez";
    let oficio = "Programadora en Rust";
    let edad = 30;
    let ciudad = "Sevilla";
    let cafe = 2.5;

    // 1. Bordes: 28 caracteres en total.
    println!("+{:-<26}+", "");
    // 2. Cabecera centrada en 24 columnas.
    println!("| {:^24} |", nombre);
    println!("| {:^24} |", oficio);
    println!("+{:-<26}+", "");
    // 3. Etiqueta a la izquierda (14), valor a la derecha (10).
    println!("| {:<14}{:>10} |", "Edad:", edad);
    println!("| {:<14}{:>10} |", "Ciudad:", ciudad);
    println!("| {:<14}{:>10.2} |", "Café al día:", cafe);
    println!("+{:-<26}+", "");
}
