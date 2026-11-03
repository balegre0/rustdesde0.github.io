fn signo(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0 // el caso que faltaba: n == 0
    }
}

fn main() {
    println!("{}", signo(-5));
}
