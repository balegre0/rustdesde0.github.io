fn main() {
    let r: Vec<u32> = (1..=10)
        .filter(|n| n % 3 == 0)
        .map(|n| n * n)
        .take(2)
        .collect();
    println!("{r:?}");
}
