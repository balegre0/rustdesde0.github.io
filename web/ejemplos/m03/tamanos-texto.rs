use std::mem::size_of;

fn main() {
    // Lo que ocupa la "ficha" en la pila, no el texto
    println!("&str   ocupa {} bytes", size_of::<&str>());
    println!("String ocupa {} bytes", size_of::<String>());
    println!("usize  ocupa {} bytes", size_of::<usize>());

    let s = String::from("año");
    println!("len = {}, capacity = {}", s.len(), s.capacity());
}
