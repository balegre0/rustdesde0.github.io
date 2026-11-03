fn suma(xs: &[i32]) -> i32 {
    let mut t = 0;
    for x in xs {
        t += x;
    }
    t
}

fn main() {
    let v = vec![1, 2, 3, 4];
    let a = [10, 20];
    println!("{} {} {}", suma(&v), suma(&v[1..3]), suma(&a));
}
