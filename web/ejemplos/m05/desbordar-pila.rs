fn cuenta_atras(n: u64) -> u64 {
    if n == 0 { 0 } else { 1 + cuenta_atras(n - 1) }
}

fn main() {
    // Mil millones de llamadas anidadas no caben en la pila.
    println!("{}", cuenta_atras(1_000_000_000));
}
