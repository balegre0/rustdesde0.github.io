fn main() {
    let mut x = 1;
    let r = &mut x;
    r += 1;
    println!("{x}");
}
