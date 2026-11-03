// Una lista: o está vacía, o tiene un número y el resto de la lista.
enum Lista {
    Nodo(i32, Lista),
    Vacia,
}

fn main() {
    let _l = Lista::Vacia;
}
