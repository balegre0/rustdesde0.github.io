fn main() {
    let mut v = vec![1, 2, 3];
    for x in &mut v {
        *x *= 10;
    }
    let mut total = 0;
    for x in &v {
        total += x;
    }
    println!("{:?} {}", v, total);
}
