fn par() -> (i32, i32) {
    (1, 2);
}

fn main() {
    let (a, b) = par();
    println!("{a} {b}");
}
