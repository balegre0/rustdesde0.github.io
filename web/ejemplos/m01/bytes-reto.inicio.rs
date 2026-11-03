use std::mem::size_of;

// Cambia el tipo para que el programa imprima: 16 bytes = 128 bits
fn main() {
    let bytes = size_of::<u8>();
    println!("{} bytes = {} bits", bytes, bytes * 8);
}
