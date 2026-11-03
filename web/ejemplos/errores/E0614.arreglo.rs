fn main() {
    let n: u32 = 5;
    let r = &n;
    let m = *r + 1; // `*` solo se aplica a referencias y punteros
    println!("{m}");
}
