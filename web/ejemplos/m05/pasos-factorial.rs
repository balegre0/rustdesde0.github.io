fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

fn main() {
    let r = factorial(3);
    println!("3! = {r}");
}
