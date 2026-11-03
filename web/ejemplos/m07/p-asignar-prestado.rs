fn main() {
    let mut n = 10;
    let r = &n;
    n = 20;
    println!("{r} {n}");
}
