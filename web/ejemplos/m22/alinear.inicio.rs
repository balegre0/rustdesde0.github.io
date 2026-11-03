// Reto: el corazón de cualquier asignador. Redondea `dir` hacia arriba
// al siguiente múltiplo de `alin` (que siempre es una potencia de dos), SIN usar
// next_multiple_of: con aritmética de bits.
fn alinear_arriba(dir: usize, alin: usize) -> usize {
    todo!()
}

fn main() {
    for (d, a) in [(0, 8), (1, 8), (8, 8), (9, 4), (13, 16), (1000, 1)] {
        println!("alinear_arriba({d:>4}, {a:>2}) = {}", alinear_arriba(d, a));
    }
}
