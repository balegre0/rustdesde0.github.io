fn main() {
    let stock: u32 = 2;
    let mut vendidos: u32 = 0;
    for _ in 0..3 {
        vendidos += 1;
    }
    let quedan = stock - vendidos;
    println!("quedan {quedan}");
}
