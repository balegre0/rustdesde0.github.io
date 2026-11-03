fn suma_pares(v: &[i32]) -> i32 {
    v.iter().filter(|n| *n % 2 == 0).sum()
}
