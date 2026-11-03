fn main() {
    // Imprime en una sola línea, separados por espacios, los números primos
    // del 2 al 30 (un primo solo es divisible por 1 y por sí mismo).
    // Pista: un `for` exterior con etiqueta, un `for` interior que prueba
    // divisores, y `continue 'etiqueta` en cuanto encuentres uno.
    for n in 2..=30 {
        print!("{n} ");
    }
    println!();
}
