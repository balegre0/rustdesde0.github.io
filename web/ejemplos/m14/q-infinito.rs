fn main() {
    let s: u64 = (1..).map(|x| x * x).take(3).sum();
    println!("{s}");
}
