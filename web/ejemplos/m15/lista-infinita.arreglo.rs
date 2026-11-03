// El resto de la lista va en una Box: un puntero de tamaño fijo.
enum Lista {
    Nodo(i32, Box<Lista>),
    Vacia,
}

fn suma(l: &Lista) -> i32 {
    match l {
        Lista::Nodo(v, resto) => v + suma(resto),
        Lista::Vacia => 0,
    }
}

fn main() {
    let l = Lista::Nodo(1, Box::new(Lista::Nodo(2, Box::new(Lista::Vacia))));
    println!("suma = {}", suma(&l));
}
