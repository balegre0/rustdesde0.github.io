enum Lista {
    Nodo(i32, Box<Lista>),
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
    println!("suma = {}", suma(&l));
    println!("una Lista ocupa {} bytes", std::mem::size_of::<Lista>());
}
