fn main() {
    let a = String::from("pan");
    let mut b = a.clone();
    b.push_str(" y queso");
    println!("{a} | {b}");
}
