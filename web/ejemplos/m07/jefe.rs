/// Quita los repetidos que vienen seguidos: [1, 1, 2, 2, 2, 3, 1] → [1, 2, 3, 1].
fn compactar(v: &mut Vec<i32>) {
    let mut i = 1;
    while i < v.len() {
        if v[i] == v[i - 1] {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}

/// Devuelve (mínimo, máximo) de una slice que no esté vacía.
fn extremos(xs: &[i32]) -> (i32, i32) {
    let mut min = xs[0];
    let mut max = xs[0];
    for &x in xs {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    (min, max)
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
