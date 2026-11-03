fn main() {
    let mut s = String::new();
    s.push('ñ');
    s.push_str("u");
    println!("{s} {} {}", s.len(), s.chars().count());
}
