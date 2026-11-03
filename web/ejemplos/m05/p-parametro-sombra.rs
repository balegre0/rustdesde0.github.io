fn doblar(x: i32) -> i32 {
    let x = x * 2;
    x + 1
}

fn main() {
    let x = 5;
    let y = doblar(x);
    println!("{x} {y}");
}
