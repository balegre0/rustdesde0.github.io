// Suma los `n` elementos que empiezan en `p` usando p.add(i).
/// # Safety
/// `p` debe apuntar al inicio de al menos `n` i32 válidos.
unsafe fn suma(p: *const i32, n: usize) -> i32 {
    let mut total = 0;
    for i in 0..n {
        let _ = i;
        todo!("suma *p.add(i)")
    }
    total
}

fn main() {
    let v = [3, 5, 7, 9];
    // SAFETY: v tiene 4 elementos.
    println!("{}", unsafe { suma(v.as_ptr(), v.len()) });
    // SAFETY: los 2 primeros existen.
    println!("{}", unsafe { suma(v.as_ptr(), 2) });
}
