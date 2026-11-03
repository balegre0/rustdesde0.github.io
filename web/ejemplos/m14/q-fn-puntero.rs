fn triple(x: i32) -> i32 {
    x * 3
}

fn main() {
    let v: Vec<i32> = [1, 2, 3].iter().map(|&x| x).map(triple).collect();
    let f: fn(i32) -> i32 = |x| x + 1;
    println!("{v:?} {}", f(1));
}
