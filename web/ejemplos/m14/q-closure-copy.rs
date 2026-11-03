fn main() {
    let x = 10;
    let f = || x + 1;
    let g = f;
    println!("{} {}", f(), g());
}
