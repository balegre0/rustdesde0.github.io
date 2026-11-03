fn main() {
    let mut v = vec![1, 2, 3];
    let primero = v[0]; // una copia del i32, no un préstamo
    v.push(4);
    println!("el primero era {primero}; ahora hay {}", v.len());
}
