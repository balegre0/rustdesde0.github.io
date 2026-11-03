fn duplicar(v: &mut Vec<i32>) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

fn main() {
    // 1. Variable mutable.
    let mut n = 1;
    n += 1;
    println!("{n}");
    // 2. Préstamo exclusivo `&mut`: permite modificar sin ser el dueño.
    let mut v = vec![1, 2, 3];
    duplicar(&mut v);
    println!("{v:?}");
    // 3. `mut` en un patrón: solo esa variable es mutable.
    let (mut a, b) = (10, 20);
    a += b;
    println!("{a} {b}");
    // 4. Puntero crudo `*mut T`.
    let mut x = 5;
    let p: *mut i32 = &mut x;
    // SAFETY: `p` apunta a `x`, viva y sin otros préstamos activos.
    unsafe { *p += 1 };
    println!("{x}");
}
