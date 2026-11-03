// El borrow checker no sabe que dos rangos de un slice no se solapan.
fn main() {
    let mut v = [1, 2, 3, 4];
    let a = &mut v[..2];
    let b = &mut v[2..];
    a[0] = 10;
    b[0] = 30;
    println!("{v:?}");
}
