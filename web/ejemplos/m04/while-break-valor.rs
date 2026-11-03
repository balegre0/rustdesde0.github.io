fn main() {
    let mut n = 1;
    let primero = while n < 100 {
        if n % 7 == 0 {
            break n;
        }
        n += 1;
    };
    println!("{primero:?}");
}
