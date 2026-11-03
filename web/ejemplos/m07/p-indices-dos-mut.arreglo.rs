fn main() {
    let mut v = vec![1, 2];
    let (izq, der) = v.split_at_mut(1);
    let a = &mut izq[0];
    let b = &mut der[0];
    *a += *b;
    println!("{v:?}");
}
