/// Quita los repetidos que vienen seguidos: [1, 1, 2, 2, 2, 3, 1] → [1, 2, 3, 1].
/// Modifica el vector a través del préstamo exclusivo: no devuelve nada.
fn compactar(v: &mut Vec<i32>) {
    todo!()
}

/// Devuelve (mínimo, máximo) de una slice que no esté vacía.
fn extremos(xs: &[i32]) -> (i32, i32) {
    todo!()
}

fn main() {
    let mut lecturas = vec![3, 3, 7, 7, 7, 1, 1, 9, 3];
    compactar(&mut lecturas);
    println!("{lecturas:?}");
    let (min, max) = extremos(&lecturas);
    println!("mín {min} · máx {max}");
    let (a, b) = extremos(&lecturas[..2]);
    println!("primeros dos: {a}..{b}");
}
