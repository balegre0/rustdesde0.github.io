use std::mem::size_of;

fn main() {
    // 1. El Vec en sí (en la pila) son tres números: puntero, capacidad y longitud.
    println!("size_of::<Vec<u32>>() = {} bytes", size_of::<Vec<u32>>());
    println!("size_of::<Vec<[u8; 100]>>() = {} bytes (¡lo mismo!)", size_of::<Vec<[u8; 100]>>());

    // 2. Cómo crece la capacidad al hacer push.
    let mut v: Vec<u32> = Vec::new();
    let mut cap = v.capacity();
    let mut puntero = v.as_ptr();
    println!("vacío: len {} cap {}", v.len(), cap);
    for i in 0..33 {
        v.push(i);
        if v.capacity() != cap {
            let movido = if v.as_ptr() != puntero { "  (los datos se han movido a otro sitio)" } else { "" };
            cap = v.capacity();
            puntero = v.as_ptr();
            println!("tras push nº {:>2}: len {:>2} cap {:>2}{}", v.len(), v.len(), cap, movido);
        }
    }

    // 3. Reservar de antemano evita repetir esas mudanzas.
    let mut w: Vec<u32> = Vec::with_capacity(100);
    println!("with_capacity(100): len {} cap {}", w.len(), w.capacity());
    for i in 0..100 {
        w.push(i);
    }
    println!("tras 100 push: len {} cap {}", w.len(), w.capacity());

    // 4. Quitar no devuelve memoria; shrink_to_fit sí.
    w.truncate(3);
    println!("truncate(3): len {} cap {}", w.len(), w.capacity());
    w.shrink_to_fit();
    println!("shrink_to_fit: len {} cap {}", w.len(), w.capacity());
}
