fn main() {
    // 1. println! acepta cualquier número de argumentos
    println!("Sin huecos");
    println!("Un hueco: {}", 1);
    println!("Tres huecos: {} {} {}", 'a', "bé", 3.5);

    // 2. vec! construye un Vec a partir de una lista
    let v = vec![10, 20, 30];
    println!("v = {:?}", v);

    // 3. stringify! convierte el código que le das en texto, sin ejecutarlo
    let texto = stringify!(1 + 2 * 3);
    println!("stringify! dice: {}", texto);
    println!("y el cálculo da: {}", 1 + 2 * 3);
}
