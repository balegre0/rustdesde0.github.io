fn cuenta(n: u32) -> u32 {
    if n <= 1 { 1 } else { 1 + cuenta(n / 2) }
}

fn main() {
    println!("{}", cuenta(100));
}
