fn main() {
    // 1. `*` antes que `+`; `as` antes que `*`.
    println!("{}", 2 + 3 * 4);
    println!("{}", 10 - 2 as i32 * 3);
    // 2. Una llamada a método va antes que el `-` unario: esto es -(2.pow(2)).
    println!("{}", -2_i32.pow(2));
    println!("{}", (-2_i32).pow(2));
    // 3. `&` (bits) va antes que `==`: esto es (6 & 3) == 2.
    println!("{}", 6 & 3 == 2);
    // 4. Asociatividad: la resta va de izquierda a derecha; la asignación, de derecha a izquierda.
    println!("{}", 10 - 4 - 3);
    let a;
    let mut b;
    a = { b = 5; b } + 1;
    println!("{a} {b}");
    b += a;
    println!("{b}");
    // 5. Los bloques son expresiones: su valor es la última expresión.
    let x = { let y = 3; y * y };
    println!("{x}");
    // 6. Orden de evaluación: de izquierda a derecha.
    let mut log = Vec::new();
    let mut f = |n: i32| { log.push(n); n };
    let _ = f(1) + f(2) * f(3);
    println!("{log:?}");
}
