use std::ops::Deref;

struct MiCaja<T>(T);

// 1. Implementar Deref le dice a Rust qué hay «dentro» de MiCaja.
impl<T> Deref for MiCaja<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let c = MiCaja(10);
    // 2. *c se convierte en *(c.deref()).
    let n = *c + 1;
    println!("n = {n}");
    // 3. Los métodos de T también se encuentran a través de Deref.
    let s = MiCaja(String::from("hola"));
    println!("largo = {}", s.len());
}
