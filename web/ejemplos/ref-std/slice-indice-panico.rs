fn main() {
    let v = [1, 2, 3];
    let fin = v.len() + 1;
    let trozo = &v[1..fin];
    println!("{trozo:?}");
}
