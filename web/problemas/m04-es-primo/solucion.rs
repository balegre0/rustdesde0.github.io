fn es_primo(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let n = n as u64;
    let mut d: u64 = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
