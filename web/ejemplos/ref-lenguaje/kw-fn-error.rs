fn sumar(a: i32, b: i32) -> i32 {
    // El `;` convierte la expresión en sentencia: la función devolvería `()`.
    a + b;
}

fn main() {
    println!("{}", sumar(1, 2));
}
