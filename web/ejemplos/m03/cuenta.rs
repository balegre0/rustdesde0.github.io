fn main() {
    let saldo: u32 = 50;
    let gasto: u32 = 80;
    println!("resta segura: {:?}", saldo.checked_sub(gasto));
    println!("saldo final: {}", saldo.saturating_sub(gasto));
}
