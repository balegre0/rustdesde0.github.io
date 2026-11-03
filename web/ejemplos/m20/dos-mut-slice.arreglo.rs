// El borrow checker no sabe que dos rangos de un slice no se solapan...
// pero split_at_mut (unsafe por dentro, segura por fuera) sí lo garantiza.
fn main() {
    let mut v = [1, 2, 3, 4];
    let (a, b) = v.split_at_mut(2);
    a[0] = 10;
    b[0] = 30;
    println!("{v:?}");
}
