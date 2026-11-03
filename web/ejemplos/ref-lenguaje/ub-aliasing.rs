fn main() {
    let mut x = 0u32;
    let p = &raw mut x;
    // Dos préstamos exclusivos del mismo valor a la vez, fabricados con un puntero crudo.
    // El compilador no puede verlo; Miri sí: rompe las reglas de aliasing.
    let a = unsafe { &mut *p };
    let b = unsafe { &mut *p };
    *a += 1;
    *b += 1;
    println!("{x}");
}
