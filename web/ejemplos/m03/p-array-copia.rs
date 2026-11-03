fn main() {
    let a = [1, 2, 3];
    let mut b = a;
    b[0] = 100;
    println!("{:?} {:?}", a, b);
}
