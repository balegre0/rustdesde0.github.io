/// Devuelve el elemento `i` sin comprobar el límite.
///
/// # Safety
/// Quien llame debe garantizar que `i < v.len()`.
unsafe fn leer_sin_comprobar(v: &[u32], i: usize) -> u32 {
    // Edición 2024: dentro de una `unsafe fn` también hace falta un bloque `unsafe`.
    // SAFETY: el llamante garantiza que `i` está dentro de `v`.
    unsafe { *v.get_unchecked(i) }
}

// Un trait `unsafe`: implementarlo es prometer algo que el compilador no comprueba.
unsafe trait SinCeros {}
unsafe impl SinCeros for std::num::NonZeroU8 {}

fn exige_sin_ceros<T: SinCeros>(_: T) -> &'static str { "promesa cumplida" }

fn main() {
    let v = [10, 20, 30];
    // 1. Llamar a una `unsafe fn` exige un bloque `unsafe`.
    // SAFETY: 2 < 3.
    let x = unsafe { leer_sin_comprobar(&v, 2) };
    println!("{x}");
    // 2. Desreferenciar un puntero crudo.
    let p = v.as_ptr();
    // SAFETY: `p.add(1)` sigue dentro del array.
    println!("{}", unsafe { *p.add(1) });
    // 3. Abstracción segura: `split_at_mut` usa unsafe por dentro, pero su API es segura.
    let mut w = [1, 2, 3, 4];
    let (a, b) = w.split_at_mut(2);
    a[0] += b[0];
    println!("{w:?}");
    println!("{}", exige_sin_ceros(std::num::NonZeroU8::new(7).unwrap()));
}
