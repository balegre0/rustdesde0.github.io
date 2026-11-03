fn suma_recursiva(n: u64) -> u64 {
    if n == 0 { 0 } else { n + suma_recursiva(n - 1) }
}

fn suma_bucle(n: u64) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

fn main() {
    // 10 000 llamadas anidadas: 10 000 marcos en la pila a la vez.
    println!("recursiva(10 000) = {}", suma_recursiva(10_000));
    // El bucle usa un solo marco, haga las vueltas que haga.
    println!("bucle(10 000 000) = {}", suma_bucle(10_000_000));
}
