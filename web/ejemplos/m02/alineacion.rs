fn main() {
    // 1. Ancho 8: el número se alinea a la derecha; el texto, a la izquierda.
    println!("[{:8}]", 42);
    println!("[{:8}]", "pan");
    // 2. < izquierda, ^ centro, > derecha.
    println!("[{:<8}]", "pan");
    println!("[{:^8}]", "pan");
    println!("[{:>8}]", "pan");
    // 3. Un carácter antes de la flecha es el relleno.
    println!("[{:*^8}]", "pan");
    println!("[{:08}]", 42);
}
