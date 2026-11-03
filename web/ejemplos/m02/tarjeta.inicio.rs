fn main() {
    let nombre = "Ana Martínez";
    let oficio = "Programadora en Rust";
    let edad = 30;
    let ciudad = "Sevilla";
    let cafe = 2.5;

    // 1. Bordes: un + , 26 guiones y otro + (pista: {:-<26} con "").
    println!("+--+");
    // 2. Nombre y oficio centrados en 24 columnas, entre "| " y " |".
    println!("| {} |", nombre);
    println!("| {} |", oficio);
    // 3. Etiqueta a la izquierda (14) y valor a la derecha (10).
    //    El café, con 2 decimales.
    println!("| {} {} |", "Edad:", edad);
    println!("| {} {} |", "Ciudad:", ciudad);
    println!("| {} {} |", "Café al día:", cafe);
}
