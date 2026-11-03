enum Lista {
    Nodo(i32, Box<Lista>),
    Vacia,
}

fn longitud(l: &Lista) -> usize {
    match l {
        Lista::Nodo(_, resto) => 1 + longitud(resto),
        Lista::Vacia => 0,
    }
}

fn maximo(l: &Lista) -> Option<i32> {
    match l {
        Lista::Nodo(valor, resto) => match maximo(resto) {
            Some(m) => Some((*valor).max(m)),
            None => Some(*valor),
        },
        Lista::Vacia => None,
    }
}
