fn main() {
    let mut x = 1;
    let f = move || x;
    x = 5;
    println!("{} {}", f(), x);
}
