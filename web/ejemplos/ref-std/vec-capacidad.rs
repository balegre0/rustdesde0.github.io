fn main() {
    let mut v: Vec<u64> = Vec::with_capacity(2);
    println!("len {} cap {}", v.len(), v.capacity());
    for i in 0..5 {
        v.push(i);
        println!("tras push({i}): len {} cap {}", v.len(), v.capacity());
    }
    v.shrink_to_fit();
    println!("tras shrink_to_fit: cap {}", v.capacity());
    // un Vec en la pila son 3 números: puntero, capacidad y longitud
    println!("tamaño de Vec en la pila: {} bytes", std::mem::size_of::<Vec<u64>>());
}
