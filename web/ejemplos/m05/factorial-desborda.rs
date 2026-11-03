fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

fn main() {
    // 20! cabe en un u64; 21! ya no
    println!("20! = {}", factorial(20));
    println!("21! = {}", factorial(21));
}
