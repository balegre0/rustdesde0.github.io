use std::mem::size_of;

enum Lista {
    Nodo(i32, Box<Lista>),
    Vacia,
}

use Lista::{Nodo, Vacia};

// 2. Recorrer la lista con recursión: cada nodo suma su número al resto.
fn suma(l: &Lista) -> i32 {
    match l {
        Nodo(valor, resto) => valor + suma(resto),
        Vacia => 0,
    }
}

fn main() {
    // 1. 1 → 2 → 3 → fin. Cada Box apunta al siguiente nodo en el heap.
    let l = Nodo(1, Box::new(Nodo(2, Box::new(Nodo(3, Box::new(Vacia))))));
    println!("suma = {}", suma(&l));
    // 3. El tamaño de Lista ya es finito y conocido.
    println!("Lista ocupa {} bytes", size_of::<Lista>());
}
