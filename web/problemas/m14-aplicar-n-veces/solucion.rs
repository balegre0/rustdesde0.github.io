fn aplicar_n(f: impl Fn(i64) -> i64, n: usize, x: i64) -> i64 {
    (0..n).fold(x, |acc, _| f(acc))
}
