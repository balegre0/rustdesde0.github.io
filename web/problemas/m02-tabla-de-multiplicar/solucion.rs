fn tabla(n: u32) -> String {
    let mut s = String::new();
    for i in 1..=10 {
        s.push_str(&format!("{:>2} x {:>2} = {:>3}\n", n, i, n * i));
    }
    s
}
