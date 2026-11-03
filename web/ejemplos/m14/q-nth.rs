fn main() {
    let mut it = [10, 20, 30, 40].into_iter();
    let a = it.nth(1);
    let b = it.next();
    println!("{a:?} {b:?}");
}
