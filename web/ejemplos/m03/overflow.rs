fn main() {
    // 1. Un u8 llega hasta 255
    let mut nivel: u8 = 253;
    // 2. Subimos de uno en uno y miramos qué pasa al pasar de 255
    for _ in 0..4 {
        println!("nivel = {nivel}");
        nivel += 1;
    }
    println!("fin: {nivel}");
}
