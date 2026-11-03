// Escribe `potencia(base, exp)` de forma RECURSIVA:
//   potencia(b, 0) = 1
//   potencia(b, e) = b * potencia(b, e - 1)
fn potencia(base: u64, exp: u32) -> u64 {
    todo!()
}

fn main() {
    println!("2^10 = {}", potencia(2, 10));
    println!("3^4 = {}", potencia(3, 4));
    println!("7^0 = {}", potencia(7, 0));
}
