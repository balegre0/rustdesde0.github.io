fn main() {
    let v: Vec<i32> = ["4", "x", "15"].iter().filter_map(|s| s.parse().ok()).collect();
    println!("{v:?}");
}
