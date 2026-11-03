fn main() {
    let mut pila = vec![1, 2, 3];
    while let Some(tope) = pila.pop() {
        print!("{tope} ");
    }
    println!();
    let quizas: Option<u8> = None;
    if let Some(n) = quizas {
        println!("hay {n}");
    } else {
        println!("no hay nada");
    }
}
