fn main() {
    let temperatura = 34;

    // 1. `if` como decisión: ejecuta una rama u otra
    if temperatura > 30 {
        println!("hace calor");
    } else {
        println!("se está bien");
    }

    // 2. `if` como expresión: produce un valor que guardamos
    let ropa = if temperatura > 30 { "camiseta" } else { "jersey" };
    println!("ponte {ropa}");

    // 3. Cadena de `else if`: se elige la primera condición verdadera
    let nivel = if temperatura >= 40 {
        3
    } else if temperatura >= 30 {
        2
    } else {
        1
    };
    println!("nivel de calor: {nivel}");
}
