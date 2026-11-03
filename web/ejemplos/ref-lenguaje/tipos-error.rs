fn main() {
    let a: i32 = 10;
    let b: i64 = 20;
    // Rust no mezcla tipos numéricos distintos sin una conversión explícita.
    println!("{}", a + b);
}
