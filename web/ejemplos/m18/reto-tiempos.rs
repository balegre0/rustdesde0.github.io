// Cada pedido: 1 tic de trabajo + su espera. Calcula el tiempo total
// haciéndolos uno tras otro y haciéndolos intercalados (el pedido i empieza en el tic i).
fn secuencial(esperas: &[u32]) -> u32 {
    esperas.iter().map(|e| 1 + e).sum()
}

fn intercalado(esperas: &[u32]) -> u32 {
    esperas
        .iter()
        .enumerate()
        .map(|(i, e)| i as u32 + 1 + e)
        .max()
        .unwrap_or(0)
}

fn main() {
    let esperas = [4, 4, 4, 4];
    println!("secuencial = {}", secuencial(&esperas));
    println!("intercalado = {}", intercalado(&esperas));
}
