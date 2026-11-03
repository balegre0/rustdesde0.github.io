fn main() {
    let n = 0;
    // Un entero no es un `bool`: la condición de un `if` debe ser `bool`.
    if n {
        println!("nunca");
    }
}
