// 1. `extern "C" fn`: una función de Rust con la convención de llamada C,
//    para que otro código compilado pueda llamarla.
extern "C" fn triple(x: i32) -> i32 {
    x * 3
}

// 2. Puntero a función con ABI explícita.
fn aplicar(f: extern "C" fn(i32) -> i32, v: i32) -> i32 {
    f(v)
}

// 3. Bloque `unsafe extern`: declara funciones definidas fuera de Rust.
//    `safe` marca las que se pueden llamar sin `unsafe`.
//    (Aquí no la llamamos: no hay biblioteca externa que la defina.)
#[allow(dead_code)]
unsafe extern "C" {
    safe fn abs(x: i32) -> i32;
}

fn main() {
    println!("{}", triple(5));
    println!("{}", aplicar(triple, 7));
}
