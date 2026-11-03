use std::mem::size_of;

fn main() {
    // 1. Un número normal vive en la pila.
    let en_pila: i32 = 5;
    // 2. Box::new pide memoria en el heap y guarda ahí el 5.
    let en_heap: Box<i32> = Box::new(5);
    // 3. Para leer el valor de dentro, se usa * (desreferenciar).
    let suma = en_pila + *en_heap;
    println!("suma = {suma}");
    // 4. Lo que hay en la pila es solo una dirección.
    println!("i32 ocupa {} bytes", size_of::<i32>());
    println!("Box<i32> ocupa {} bytes", size_of::<Box<i32>>());
    println!("Box<[u8; 1000]> ocupa {} bytes", size_of::<Box<[u8; 1000]>>());
}
// 5. Al salir de main, en_heap se destruye y libera su memoria del heap.
