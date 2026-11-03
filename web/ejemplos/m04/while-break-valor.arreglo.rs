fn main() {
    let mut n = 1;
    // Solo `loop` puede entregar un valor con `break`
    let primero = loop {
        if n % 7 == 0 {
            break n;
        }
        n += 1;
    };
    println!("{primero}");
}
