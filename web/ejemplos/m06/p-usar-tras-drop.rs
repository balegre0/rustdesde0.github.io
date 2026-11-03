fn main() {
    let v = vec![1, 2, 3];
    drop(v);
    println!("{}", v.len());
}
