fn main() {
    // 1. `if` / `else if` / `else` como expresión: da un valor.
    let t = 18;
    let ropa = if t < 10 { "abrigo" } else if t < 20 { "chaqueta" } else { "camiseta" };
    println!("{ropa}");
    // 2. `if let ... else`.
    let o: Option<i32> = None;
    if let Some(x) = o { println!("{x}"); } else { println!("no hay valor"); }
    // 3. `let ... else`: si el patrón no encaja, el `else` debe salir (divergir).
    let texto = "42";
    let Ok(n) = texto.parse::<i32>() else {
        println!("no es un número");
        return;
    };
    println!("n + 1 = {}", n + 1);
}
