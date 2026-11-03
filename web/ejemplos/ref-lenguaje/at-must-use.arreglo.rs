#![deny(unused_must_use)]

#[must_use = "la suma no modifica nada: guarda el resultado"]
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let s = sumar(2, 3);
    println!("{s}");
    let _ = sumar(1, 1); // descartar a propósito también vale
}
