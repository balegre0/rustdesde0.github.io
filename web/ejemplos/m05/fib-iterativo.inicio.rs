// Calcula Fibonacci SIN recursión: fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2).
// Pista: guarda los dos últimos valores en dos variables y avanza en un bucle.
fn fib(n: u32) -> u64 {
    let _ = n;
    todo!()
}

fn main() {
    for n in [0, 1, 2, 10, 50, 90] {
        println!("fib({n}) = {}", fib(n));
    }
}
