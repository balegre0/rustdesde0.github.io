enum Lista {
    Nodo(i32, Lista),
    Fin,
}

fn main() {
    let _l = Lista::Nodo(1, Lista::Fin);
}
