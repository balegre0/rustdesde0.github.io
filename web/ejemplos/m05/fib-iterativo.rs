fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let siguiente = a + b;
        a = b;
        b = siguiente;
    }
    a
}

fn main() {
    for n in [0, 1, 2, 10, 50, 90] {
        println!("fib({n}) = {}", fib(n));
    }
}
