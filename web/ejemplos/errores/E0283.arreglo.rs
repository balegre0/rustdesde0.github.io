fn main() {
    let total: u32 = "1 2 3".split(' ').map(|t| t.parse::<u32>().unwrap()).sum();
    println!("{total}");
}
