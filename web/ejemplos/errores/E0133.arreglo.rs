fn main() {
    let x = 42;
    let p = &raw const x;
    // SEGURIDAD: `p` apunta a `x`, que está viva e inicializada.
    let leido = unsafe { *p };
    println!("{leido}");
}
