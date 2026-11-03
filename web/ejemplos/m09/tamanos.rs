use std::mem::size_of;

fn main() {
    println!("Option<u16>: {} bytes", size_of::<Option<u16>>());
    // El tamaño de una referencia depende de la máquina (4 bytes en wasm32, 8 en un PC de 64 bits),
    // así que comparamos en vez de imprimir el número.
    println!("Option<&u16> mide lo mismo que &u16: {}", size_of::<Option<&u16>>() == size_of::<&u16>());
}
