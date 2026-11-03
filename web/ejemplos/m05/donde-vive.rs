fn main() {
    // 1. Un entero: vive entero en la pila
    let edad: u32 = 30;

    // 2. Un literal de texto: sus bytes están grabados en el propio programa;
    //    `literal` (en la pila) es un puntero + una longitud que los señala
    let literal: &str = "hola";

    // 3. Un String: su «ficha» (puntero, capacidad, longitud) está en la pila;
    //    sus bytes, en el heap
    let texto = String::from("hola");

    println!("pila:  edad          en {:p}", &edad);
    println!("pila:  literal       en {:p}", &literal);
    println!("pila:  texto (ficha) en {:p}", &texto);
    println!("programa: bytes de literal en {:p}", literal.as_ptr());
    println!("heap:     bytes de texto   en {:p}", texto.as_ptr());

    println!("tamaño de la ficha de un String: {} bytes", size_of::<String>());
    println!("tamaño de un &str (puntero + longitud): {} bytes", size_of::<&str>());
    println!("tamaño de un usize: {} bytes", size_of::<usize>());
    println!("bytes de texto en el heap: {} (capacidad {})", texto.len(), texto.capacity());
}
