fn main() {
    let n = 5;
    let f: fn(i32) -> i32 = |x| x + n;
    println!("{}", f(1));
}
