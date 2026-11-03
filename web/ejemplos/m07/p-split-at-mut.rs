fn main() {
    let mut v = [1, 2, 3, 4];
    let (izq, der) = v.split_at_mut(2);
    izq[0] += der[1];
    der[0] *= 10;
    println!("{v:?}");
}
