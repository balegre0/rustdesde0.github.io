fn dias_mes(n: u32) -> u32 {
    match n {
        1 => 31,
        2 => 28,
    }
}

fn main() {
    println!("{}", dias_mes(1));
}
