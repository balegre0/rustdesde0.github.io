// Completa `multiplos`: debe devolver un iterador con los `cuantos`
// primeros múltiplos de `k` (empezando por k).
// Pista: (1..=cuantos).map(...) y un closure con `move`.
fn multiplos(k: u32, cuantos: u32) -> impl Iterator<Item = u32> {
    let _ = (k, cuantos);
    std::iter::empty()
}

fn main() {
    let v: Vec<u32> = multiplos(3, 5).collect();
    println!("{v:?}");
    println!("{}", multiplos(10, 4).sum::<u32>());
}
