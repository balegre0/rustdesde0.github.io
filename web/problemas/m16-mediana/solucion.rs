fn mediana(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut copia = v.to_vec();
    copia.sort_by(|a, b| a.total_cmp(b));
    let n = copia.len();
    if n % 2 == 1 {
        Some(copia[n / 2])
    } else {
        Some((copia[n / 2 - 1] + copia[n / 2]) / 2.0)
    }
}
