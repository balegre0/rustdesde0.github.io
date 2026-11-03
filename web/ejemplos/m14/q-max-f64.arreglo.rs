fn main() {
    let v = vec![2.5, 9.1, 4.0];
    // f64 no tiene orden total (NaN); fold con f64::max sí funciona.
    let m = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    println!("{m}");
}
