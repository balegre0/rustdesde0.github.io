// 1. `-> i32` dice qué tipo devuelve. El valor es la última expresión, sin `;`
fn cuadrado(x: i32) -> i32 {
    x * x
}

fn es_par(n: u32) -> bool {
    n % 2 == 0
}

// 2. `return` sale ANTES de llegar al final
fn precio_entrada(edad: u32) -> u32 {
    if edad < 4 {
        return 0; // gratis: no hace falta seguir
    }
    if edad >= 65 { 5 } else { 9 }
}

fn main() {
    let c = cuadrado(12);
    println!("12 al cuadrado = {c}");
    println!("¿7 es par? {}", es_par(7));
    // 3. Una llamada es una expresión: se puede usar dentro de otra
    println!("cuadrado(cuadrado(3)) = {}", cuadrado(cuadrado(3)));
    for edad in [2, 30, 70] {
        println!("entrada para {edad} años: {} €", precio_entrada(edad));
    }
}
