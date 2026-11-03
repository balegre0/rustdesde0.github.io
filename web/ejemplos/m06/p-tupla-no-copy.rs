fn main() {
    let a = (1, String::from("x"));
    let b = a;
    println!("{} {}", a.0, b.1);
}
