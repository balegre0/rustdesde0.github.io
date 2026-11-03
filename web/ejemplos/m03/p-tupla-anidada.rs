fn main() {
    let t = ((1, 2), (3, 4));
    println!("{}", t.1.0);
    let ((a, _), (_, d)) = t;
    println!("{}", a + d);
}
