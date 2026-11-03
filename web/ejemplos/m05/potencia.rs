fn potencia(base: u64, exp: u32) -> u64 {
    if exp == 0 { 1 } else { base * potencia(base, exp - 1) }
}

fn main() {
    println!("2^10 = {}", potencia(2, 10));
    println!("3^4 = {}", potencia(3, 4));
    println!("7^0 = {}", potencia(7, 0));
}
