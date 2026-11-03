fn main() {
    let mut n = 1;
    let mut pasos = 0;
    while n < 100 {
        n *= 3;
        pasos += 1;
    }
    println!("{n} {pasos}");
}
