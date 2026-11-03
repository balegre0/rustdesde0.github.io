fn main() {
    let mut x = 1;
    let r = &mut x;
    println!("{x}");
    *r += 1;
}
