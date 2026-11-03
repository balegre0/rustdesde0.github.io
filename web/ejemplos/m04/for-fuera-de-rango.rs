fn main() {
    let notas = [7, 9, 4, 10];
    // ❌ `0..=notas.len()` llega hasta 4, y la última posición válida es 3
    for i in 0..=notas.len() {
        println!("nota {i}: {}", notas[i]);
    }
}
