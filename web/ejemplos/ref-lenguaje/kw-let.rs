fn main() {
    // 1. Declarar una variable (inmutable por defecto).
    let x = 5;
    // 2. Con tipo explícito y mutable.
    let mut total: i64 = 0;
    total += x;
    println!("{total}");
    // 3. Sombreado (shadowing): un `let` nuevo con el mismo nombre.
    let x = x * 10;
    println!("{x}");
    // 4. Desestructurar con un patrón.
    let (a, b) = (1, "uno");
    println!("{a} = {b}");
    // 5. Inicialización diferida: el compilador comprueba que se asigna antes de leer.
    let mensaje;
    if x > 10 { mensaje = "grande"; } else { mensaje = "pequeño"; }
    println!("{mensaje}");
    // 6. `let ... else`.
    let Some(primero) = "abc".chars().next() else { return };
    println!("{primero}");
}
