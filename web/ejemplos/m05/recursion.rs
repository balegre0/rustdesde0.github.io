// 1. Caso base (n == 0) + caso recursivo (n * factorial de n - 1)
fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

// 2. Una función que se llama a sí misma y muestra su profundidad
fn cuenta_atras(n: u32) {
    if n == 0 {
        println!("¡despegue!");
        return;
    }
    println!("{n}...");
    cuenta_atras(n - 1);
    println!("vuelvo de cuenta_atras({})", n - 1);
}

fn main() {
    for n in [0, 1, 5, 20] {
        println!("{n}! = {}", factorial(n));
    }
    cuenta_atras(3);
}
