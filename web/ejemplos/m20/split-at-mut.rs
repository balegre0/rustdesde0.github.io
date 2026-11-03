// Una abstracción segura sobre unsafe: partir un slice en dos mitades exclusivas.
fn partir_mut(s: &mut [i32], mitad: usize) -> (&mut [i32], &mut [i32]) {
    let largo = s.len();
    // 1. La comprobación que hace segura la función. Sin ella, sería unsound.
    assert!(mitad <= largo, "mitad fuera de rango");
    let p = s.as_mut_ptr();
    // SAFETY: mitad <= largo, así que [0, mitad) y [mitad, largo) están dentro
    // del slice y NO se solapan: los dos préstamos exclusivos son disjuntos.
    unsafe {
        (
            std::slice::from_raw_parts_mut(p, mitad),
            std::slice::from_raw_parts_mut(p.add(mitad), largo - mitad),
        )
    }
}

fn main() {
    let mut v = [1, 2, 3, 4, 5];
    let (a, b) = partir_mut(&mut v, 2);
    a[0] = 10; // 2. Los dos lados se usan a la vez
    b[0] = 30;
    println!("a = {a:?}, b = {b:?}");
    println!("v = {v:?}");

    // 3. La versión de la biblioteca estándar hace lo mismo.
    let (x, y) = v.split_at_mut(4);
    std::mem::swap(&mut x[0], &mut y[0]);
    println!("tras swap: {v:?}");
}
