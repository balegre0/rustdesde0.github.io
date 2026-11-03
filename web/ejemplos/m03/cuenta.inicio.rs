fn main() {
    let saldo: u32 = 50;
    let gasto: u32 = 80;
    // Usa checked_sub para que, en vez de un pánico, el programa imprima:
    // resta segura: None
    // y luego saturating_sub para imprimir:
    // saldo final: 0
    println!("resta segura: ?");
    println!("saldo final: ?");
    let _ = (saldo, gasto);
}
