fn main() {
    let mut v = vec![1, 2, 3];
    // Dos préstamos exclusivos a la vez: prohibido.
    let a = &mut v;
    let b = &mut v;
    a.push(4);
    b.push(5);
    println!("{v:?}");
}
