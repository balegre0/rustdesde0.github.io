fn suma_cifras(n: u32) -> u32 {
    if n < 10 { n } else { n % 10 + suma_cifras(n / 10) }
}

fn main() {
    println!("{}", suma_cifras(4096));
}
