fn main() {
    let frutas = vec!["pera", "kiwi", "piña"];
    // 1. Pedimos un iterador: todavía no ha hecho nada.
    let mut it = frutas.iter();
    // 2. Cada `next()` entrega el siguiente elemento envuelto en Some...
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    // 3. ...y cuando se acaba, None (para siempre).
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
