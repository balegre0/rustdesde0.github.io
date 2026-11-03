fn signo(x: i32) -> &'static str {
    if x < 0 {
        return -1;
    }
    "positivo"
}

fn main() {
    println!("{}", signo(3));
}
