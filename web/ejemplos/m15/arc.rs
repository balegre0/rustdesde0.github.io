use std::sync::Arc;

fn main() {
    // 1. Arc funciona como Rc, pero su contador es atómico.
    let a = Arc::new(String::from("datos de solo lectura"));
    let b = Arc::clone(&a);
    println!("dueños = {}", Arc::strong_count(&a));
    println!("b ve: {b}");
    drop(b);
    println!("dueños tras soltar b = {}", Arc::strong_count(&a));
    // 2. Con un único dueño, se puede recuperar el valor de dentro.
    match Arc::try_unwrap(a) {
        Ok(s) => println!("recuperado: {s}"),
        Err(_) => println!("todavía compartido"),
    }
}
