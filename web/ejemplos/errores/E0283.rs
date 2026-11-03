fn main() {
    let total = "1 2 3".split(' ').map(|t| t.parse::<u32>().unwrap()).sum();
    println!("{total}");
}
