// Devuelve un iterador (sin construir ningún Vec) con los múltiplos de `n`
// que hay entre 1 y `hasta`, ambos incluidos.
fn multiplos(n: u32, hasta: u32) -> impl Iterator<Item = u32> {
    // Pista: un rango, un filter y `move` (el closure usa `n`).
    std::iter::empty() // sustituye esta línea
}

fn main() {
    println!("{:?}", multiplos(3, 20).collect::<Vec<_>>());
    println!("{}", multiplos(7, 100).count());
    println!("{:?}", multiplos(5, 4).next());
}
