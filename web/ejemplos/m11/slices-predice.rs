fn suma(numeros: &[i32]) -> i32 {
    let mut total = 0;
    for n in numeros {
        total += n;
    }
    total
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let a = [10, 20, 30];
    println!("{} {} {}", suma(&v), suma(&v[1..3]), suma(&a));
}
