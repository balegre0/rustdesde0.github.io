fn main() {
    let v = vec![1, 2, 3];
    let it = v.iter();
    let a = it.count();
    let b = it.count();
    println!("{a} {b}");
}
