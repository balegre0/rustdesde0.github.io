fn main() {
    let mut n = 100;
    let pasos = loop {
        if n < 10 {
            break n;
        }
        n /= 3;
    };
    println!("{pasos}");
}
