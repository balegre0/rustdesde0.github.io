fn main() {
    let mut suma = 0;
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        suma += i;
    }
    println!("{suma}");
}
