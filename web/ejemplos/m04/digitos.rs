fn main() {
    let mut n: u32 = 90_417;
    let mut suma = 0;
    let mut cifras = 0;
    while n > 0 {
        suma += n % 10;
        n /= 10;
        cifras += 1;
    }
    println!("{cifras} cifras, suman {suma}");
}
