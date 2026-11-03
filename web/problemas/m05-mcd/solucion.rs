fn mcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { mcd(b, a % b) }
}
