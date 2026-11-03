fn invertir(v: &mut [String]) {
    let n = v.len();
    for i in 0..n / 2 {
        v.swap(i, n - 1 - i);
    }
}
