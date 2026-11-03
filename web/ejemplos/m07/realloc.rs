fn main() {
    let mut v: Vec<u64> = Vec::with_capacity(2);
    let vecino = vec![0u8; 64]; // otro bloque del heap, justo detrás de v
    v.push(1);
    v.push(2);
    let antes = v.as_ptr();
    println!("capacidad {} · datos en {:p}", v.capacity(), antes);
    v.push(3); // no cabe: Vec pide un bloque nuevo, copia y LIBERA el viejo
    let despues = v.as_ptr();
    println!("capacidad {} · datos en {:p}", v.capacity(), despues);
    println!("¿se han movido los datos? {}", antes != despues);
    println!("(el vecino sigue ahí: {} bytes)", vecino.len());
}
