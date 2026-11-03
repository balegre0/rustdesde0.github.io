fn main() {
    let n = 5;
    println!("{}", matches!(n, 1..=4 | 6));
    println!("{}", matches!(n, 1..=9 if n % 2 == 1));
    let o: Option<i32> = Some(3);
    println!("{}", matches!(o, Some(x) if x > 2));
}
