fn main() {
    const S: &str = concat!("v", 1, '.', 2, "-", true);
    println!("{}", S);
    println!("{}", S.len());
}
