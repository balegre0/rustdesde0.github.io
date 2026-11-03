// Devuelve un iterador (sin construir ningún Vec) con los múltiplos de `n`
// que hay entre 1 y `hasta`, ambos incluidos.
fn multiplos(n: u32, hasta: u32) -> impl Iterator<Item = u32> {
    (1..=hasta).filter(move |x| x % n == 0)
}

fn main() {
    println!("{:?}", multiplos(3, 20).collect::<Vec<_>>());
    println!("{}", multiplos(7, 100).count());
    println!("{:?}", multiplos(5, 4).next());
}
