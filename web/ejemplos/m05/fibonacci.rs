// Dos llamadas recursivas por llamada
fn fib(n: u32) -> u64 {
    if n < 2 { n as u64 } else { fib(n - 1) + fib(n - 2) }
}

// ¿Cuántas llamadas hace fib(n) en total? La misma forma recursiva, contando
fn llamadas(n: u32) -> u64 {
    if n < 2 { 1 } else { 1 + llamadas(n - 1) + llamadas(n - 2) }
}

fn main() {
    for n in [5, 10, 20, 25] {
        println!("fib({n}) = {} con {} llamadas", fib(n), llamadas(n));
    }
}
