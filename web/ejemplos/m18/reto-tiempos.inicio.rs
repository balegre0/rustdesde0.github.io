// Cada pedido: 1 tic de trabajo + su espera. Calcula el tiempo total
// haciéndolos uno tras otro y haciéndolos intercalados (el pedido i empieza en el tic i).
fn secuencial(esperas: &[u32]) -> u32 {
    todo!()
}

fn intercalado(esperas: &[u32]) -> u32 {
    todo!()
}

fn main() {
    let esperas = [4, 4, 4, 4];
    println!("secuencial = {}", secuencial(&esperas));
    println!("intercalado = {}", intercalado(&esperas));
}
