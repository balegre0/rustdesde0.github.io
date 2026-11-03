fn a_binario(n: u8) -> String {
    let mut s = String::new();
    for i in (0..8).rev() {
        s.push(if (n >> i) & 1 == 1 { '1' } else { '0' });
    }
    s
}
