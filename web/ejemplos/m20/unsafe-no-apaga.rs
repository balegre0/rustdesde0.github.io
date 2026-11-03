// `unsafe` no desactiva el borrow checker.
fn main() {
    let mut n = 5;
    let p = &raw mut n;
    unsafe {
        *p += 1; // esto sí necesita unsafe
        let a = &mut n;
        let b = &mut n; // ¿dos préstamos exclusivos a la vez?
        *a += 1;
        *b += 1;
    }
    println!("{n}");
}
