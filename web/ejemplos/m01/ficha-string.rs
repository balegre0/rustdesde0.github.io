use std::mem::size_of;

// La ficha de un String son tres números del tamaño de una dirección.
fn main() {
    // 1. Cuánto mide cada pieza en esta máquina
    println!("usize:  {} bytes", size_of::<usize>());
    println!("String: {} bytes = 3 x {}", size_of::<String>(), size_of::<usize>());

    // 2. Las tres piezas de la ficha
    let s = String::from("hola");
    println!("puntero:   {:p}", s.as_ptr());
    println!("longitud:  {}", s.len());
    println!("capacidad: {}", s.capacity());

    // 3. Un Box<u32> es solo un puntero al heap
    let caja = Box::new(7u32);
    println!("Box<u32>: {} bytes en la pila; el 7 vive en {:p}", size_of::<Box<u32>>(), caja);
}
