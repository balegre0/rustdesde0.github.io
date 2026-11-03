fn signo(n: i32) -> i32 {
    if n > 0 {
        return 1;
    } else if n < 0 {
        return -1;
    }
}

fn main() {
    println!("{}", signo(-5));
}
