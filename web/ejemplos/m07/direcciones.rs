fn main() {
    let s = String::from("hola");
    let r = &s;
    // {:p} imprime la dirección a la que apunta una referencia.
    println!("dirección de s en la pila: {:p}", &s);
    println!("valor guardado en r:       {:p}", r);
    println!("¿r apunta a s? {}", std::ptr::eq(r, &s));
    println!("bytes de una referencia: {}", size_of::<&String>());
    println!("bytes de la ficha de s:  {}", size_of::<String>());
}
