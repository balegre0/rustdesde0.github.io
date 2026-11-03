use std::mem::size_of;

fn main() {
    // 1. Tamaños: f32 = 4 bytes, f64 = 8 bytes
    println!("f32: {} bytes, f64: {} bytes", size_of::<f32>(), size_of::<f64>());
    // 2. Lo que de verdad se guarda para 0.1 (no es 0.1 exacto)
    println!("0.1 con 20 decimales: {:.20}", 0.1);
    println!("bits de 0.1: {:#018x}", 0.1f64.to_bits());
    // 3. Los límites
    println!("f64::MAX = {:e}", f64::MAX);
    println!("f64::EPSILON = {:e}", f64::EPSILON);
}
