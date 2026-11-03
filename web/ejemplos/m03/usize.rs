use std::mem::size_of;

fn main() {
    println!("u8    ocupa {} byte", size_of::<u8>());
    println!("i32   ocupa {} bytes", size_of::<i32>());
    println!("u64   ocupa {} bytes", size_of::<u64>());
    println!("i128  ocupa {} bytes", size_of::<i128>());
    // usize/isize: lo que mide una dirección de memoria en ESTA máquina
    println!("usize ocupa {} bytes", size_of::<usize>());
    println!("usize::MAX = {}", usize::MAX);
}
