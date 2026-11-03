// 1. Un comentario de línea empieza con // y llega hasta el final de la línea.
fn main() {
    println!("Uno"); // también puede ir detrás del código
    // println!("Dos");   <- comentado: no se ejecuta
    /* 2. Un comentario de bloque
       puede ocupar varias líneas. */
    println!("Tres");
    println!("Cuatro /* esto no es un comentario */");
}
