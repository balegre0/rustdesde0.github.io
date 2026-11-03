fn main() {
    let a: i32 = 10;
    let b: i64 = 20;
    // `i64::from` no puede perder información: un i32 siempre cabe en un i64.
    println!("{}", i64::from(a) + b);
}
