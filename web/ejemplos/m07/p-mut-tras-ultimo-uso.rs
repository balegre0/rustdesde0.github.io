fn main() {
    let mut x = 5;
    let r = &mut x;
    *r += 1;
    let s = &x;
    println!("{s}");
}
