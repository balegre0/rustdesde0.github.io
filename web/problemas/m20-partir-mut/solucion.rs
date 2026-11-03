fn partir_mut(s: &mut [i32], i: usize) -> (&mut [i32], &mut [i32]) {
    let largo = s.len();
    assert!(i <= largo, "índice fuera de rango");
    let p = s.as_mut_ptr();
    // SAFETY: i <= largo, así que [0, i) y [i, largo) están dentro de `s`
    // y no se solapan; ambos préstamos heredan la vida de `s`.
    unsafe {
        (
            std::slice::from_raw_parts_mut(p, i),
            std::slice::from_raw_parts_mut(p.add(i), largo - i),
        )
    }
}
