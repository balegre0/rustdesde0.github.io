#![deny(unused_must_use)]

#[must_use = "la suma no modifica nada: guarda el resultado"]
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    sumar(2, 3);
}
