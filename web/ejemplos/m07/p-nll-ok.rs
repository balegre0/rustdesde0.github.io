fn main() {
    let mut v = vec![1, 2, 3];
    let primero = &v[0];
    println!("primero = {primero}");
    v.push(4);
    println!("{v:?}");
}
