enum Lista {
    Nodo(i32, Box<Lista>), // un Box mide siempre lo mismo: un puntero
    Fin,
}

fn suma(l: &Lista) -> i32 {
    match l {
        Lista::Nodo(v, resto) => v + suma(resto),
        Lista::Fin => 0,
    }
}

fn main() {
    let l = Lista::Nodo(1, Box::new(Lista::Nodo(2, Box::new(Lista::Fin))));
    println!("{}", suma(&l));
}
