use std::any::type_name;
use std::ops::Add;

// 1. Una sola función genérica en el código fuente…
fn doble<T: Add<Output = T> + Copy>(x: T) -> T {
    // 2. …de la que el compilador genera una copia por cada tipo concreto.
    println!("  (copia para {})", type_name::<T>());
    x + x
}

fn main() {
    // 3. Tres tipos distintos → tres funciones distintas en el ejecutable.
    println!("{}", doble(21));
    println!("{}", doble(1.25));
    println!("{}", doble(200u8 / 2));
    // 4. Repetir un tipo NO crea otra copia: se reutiliza la de i32.
    println!("{}", doble(-5));
}
