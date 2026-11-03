fn main() {
    // 1. `if` como expresión: ambas ramas dan un valor del mismo tipo.
    let n = -4;
    let signo = if n < 0 { "negativo" } else { "positivo o cero" };
    println!("{signo}");
    // 2. `if let`: comprobar un patrón.
    let o = Some(3);
    if let Some(x) = o {
        println!("hay un {x}");
    }
    // 3. Cadenas `let` con `&&` (edición 2024, desde Rust 1.88).
    if let Some(x) = o && x > 2 {
        println!("{x} es mayor que 2");
    }
    // 4. Guarda `if` dentro de un `match`.
    match n {
        x if x % 2 == 0 => println!("{x} es par"),
        _ => println!("impar"),
    }
}
