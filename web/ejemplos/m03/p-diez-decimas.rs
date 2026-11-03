fn main() {
    let mut total = 0.0;
    for _ in 0..10 {
        total += 0.1;
    }
    println!("{total}");
    println!("{}", total == 1.0);
}
