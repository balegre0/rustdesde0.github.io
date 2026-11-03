fn main() {
    let mut v = vec![1, 2, 3];
    for x in &mut v {
        *x *= 2;
    }
    println!("{v:?}");
}
