use std::mem::size_of;

// Un número vive en la pila. Un String tiene una parte en la pila y otra en el heap.
fn main() {
    // 1. Un número: sus 4 bytes están en la pila
    let edad: u32 = 30;
    println!("edad ocupa {} bytes; vive en {:p}", size_of::<u32>(), &edad);

    // 2. Un String: una ficha en la pila...
    let mut nombre = String::from("Ana");
    println!("la ficha de nombre ocupa {} bytes; vive en {:p}", size_of::<String>(), &nombre);

    // 3. ...que apunta a las letras, que están en el heap
    println!("las letras viven en {:p}", nombre.as_ptr());
    println!("longitud {}, capacidad {}", nombre.len(), nombre.capacity());

    // 4. El texto crece: el heap guarda más letras; la ficha sigue midiendo lo mismo
    nombre.push_str(" García López");
    println!("longitud {}, capacidad {}", nombre.len(), nombre.capacity());
    println!("la ficha sigue ocupando {} bytes", size_of::<String>());
}
