fn fibonacci(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let siguiente = a + b;
        a = b;
        b = siguiente;
    }
    a
}
