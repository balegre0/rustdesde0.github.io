fn doble(mut n: i32) -> i32 {
    // 2. `n` es una copia: cambiarla no toca la `x` de main
    n *= 2;
    n
}

fn main() {
    let x = 21;
    // 1. `i32` es Copy: se pasa una copia y `x` sigue siendo válida
    let y = doble(x);
    println!("x = {x}, y = {y}");
}
