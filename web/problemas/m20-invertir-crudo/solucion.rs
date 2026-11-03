fn invertir<T>(v: &mut [T]) {
    let largo = v.len();
    if largo < 2 {
        return;
    }
    let mut i = v.as_mut_ptr();
    // SAFETY: largo >= 2, así que largo - 1 es un índice válido.
    let mut j = unsafe { i.add(largo - 1) };
    while i < j {
        // SAFETY: i y j están dentro de `v` y son distintos (i < j);
        // ptr::swap intercambia los valores sin destruir ni duplicar.
        unsafe {
            std::ptr::swap(i, j);
            i = i.add(1);
            j = j.sub(1);
        }
    }
}
