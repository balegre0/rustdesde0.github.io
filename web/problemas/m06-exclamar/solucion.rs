fn exclamar(mut s: String, n: usize) -> String {
    for _ in 0..n {
        s.push('!');
    }
    s
}
