fn main() {
    let v = vec![1, 2, 3];
    // `for x in v` mueve el vector: después ya no se puede usar.
    for x in v {
        println!("{x}");
    }
    println!("{}", v.len());
}
